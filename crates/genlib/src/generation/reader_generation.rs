use std::collections::BTreeMap;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, FieldSet, IfBranch, NestedSwitch, ProtocolType},
    util::format_hex_value,
};

/// Generate a single file containing both type definition and reader implementation
pub fn generate_type_and_reader_file(
    gen_ctx: &super::context::GenerationContext,
    reader_ctx: &super::context::ReaderContext,
    protocol_type: &ProtocolType,
) -> String {
    let mut out = String::new();

    // Add imports
    out.push_str("use serde::{Serialize, Deserialize};\n");
    out.push_str("use std::io::Read;\n");
    out.push_str("use crate::readers::ACReader;\n");
    out.push_str("use crate::readers::*;\n");
    out.push_str("use crate::types::*;\n");
    out.push_str("use crate::enums::*;\n");
    out.push_str("use super::*;\n\n");

    // Generate type definition
    if protocol_type.is_primitive {
        let type_name = &protocol_type.name;
        let rust_type = get_rust_type(type_name);
        if rust_type != type_name {
            if let Some(ref text) = protocol_type.text {
                out.push_str(&format!("/// {text}\n"));
            }
            out.push_str(&format!(
                "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
            ));
        }
    } else {
        out.push_str(&super::type_generation::generate_type(protocol_type));
    }

    // Generate reader implementation if requested
    if gen_ctx.should_generate_reader(&protocol_type.name) {
        out.push_str(&generate_reader_impl(reader_ctx, protocol_type));
    }

    out
}

/// Generate a reader implementation for a single type (as an impl block on the type)
pub fn generate_reader_impl(
    ctx: &super::context::ReaderContext,
    protocol_type: &ProtocolType,
) -> String {
    let type_name = &protocol_type.name;
    let safe_type_name = safe_identifier(type_name, IdentifierType::Type);

    // Primitive types (u32, bool, etc.) have read_* helper functions
    if protocol_type.is_primitive {
        return String::new();
    }

    // Templated/generic types (PackableList<T>, PackableHashTable<T,U>, etc.)
    // use generic helper functions like read_packable_list() instead of impl blocks
    if protocol_type.templated.is_some() {
        return String::new();
    }

    // Handle newtype structs (parent type with no fields, but not C-style aliases)
    // These are generated as newtype structs and need a read() method
    if protocol_type.parent.is_some() && protocol_type.fields.is_none() {
        if let Some(parent_type) = &protocol_type.parent {
            let rust_type = get_rust_type(parent_type);

            if crate::type_utils::should_be_newtype_struct(&safe_type_name.name, rust_type) {
                // Generate read() for newtype struct
                // Strategy: Try to use a read_* helper function
                // - For numeric primitives: use read_u32(), read_i16(), etc.
                // - For String/WString types: use read_{typename}() helper
                let read_call = match rust_type {
                    "u8" => "read_u8(reader)".to_string(),
                    "i8" => "read_i8(reader)".to_string(),
                    "u16" => "read_u16(reader)".to_string(),
                    "i16" => "read_i16(reader)".to_string(),
                    "u32" => "read_u32(reader)".to_string(),
                    "i32" => "read_i32(reader)".to_string(),
                    "u64" => "read_u64(reader)".to_string(),
                    "i64" => "read_i64(reader)".to_string(),
                    "f32" => "read_f32(reader)".to_string(),
                    "f64" => "read_f64(reader)".to_string(),
                    "bool" => "read_bool(reader)".to_string(),
                    // String or custom parent: call read_{lowercase_typename}()
                    _ => format!("read_{}(reader)", safe_type_name.name.to_lowercase()),
                };

                let impl_code = format!(
                    "impl {} {{\n    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self({}?))\n    }}\n}}\n\n",
                    safe_type_name.name, read_call
                );
                let acdatatype_code = format!(
                    "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
                    safe_type_name.name, safe_type_name.name
                );
                return format!("{}{}", impl_code, acdatatype_code);
            }
        }
        // Type alias - doesn't need a reader
        return String::new();
    }

    let Some(field_set) = &protocol_type.fields else {
        // Empty struct - no fields to read
        let impl_code = format!(
            "impl {} {{\n    pub fn read(_reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        Ok(Self {{}})\n    }}\n}}\n\n",
            safe_type_name.name
        );
        let acdatatype_code = format!(
            "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
            safe_type_name.name, safe_type_name.name
        );
        return format!("{}{}", impl_code, acdatatype_code);
    };

    // Check if this is a variant type (has switch)
    if let Some(ref variant_fields) = field_set.variant_fields {
        // For variant types with the new standalone struct approach,
        // we generate readers for each variant struct instead of a monolithic reader.
        generate_variant_struct_readers(ctx, &safe_type_name.name, field_set, variant_fields)
    } else {
        generate_struct_reader_impl(ctx, protocol_type, &safe_type_name.name, field_set)
    }
}

/// Generate readers for all variant structs of an enum type
fn generate_variant_struct_readers(
    ctx: &super::context::ReaderContext,
    type_name: &str,
    field_set: &FieldSet,
    variant_fields: &BTreeMap<i64, Vec<Field>>,
) -> String {
    let mut out = String::new();

    // Collect all case values from both variant_fields and nested_switches
    let mut all_case_values = std::collections::BTreeSet::new();
    for case_value in variant_fields.keys() {
        all_case_values.insert(*case_value);
    }
    if let Some(ref nested_switches) = field_set.nested_switches {
        for case_value in nested_switches.keys() {
            all_case_values.insert(*case_value);
        }
    }

    // Group case values by field signature (same as type generator)
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

    for case_value in &all_case_values {
        let case_fields = variant_fields.get(case_value).cloned().unwrap_or_default();
        let mut field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        // Include nested switch structure in signature to avoid grouping cases with different nested switches
        if let Some(ref nested_switches) = field_set.nested_switches
            && let Some(nested_switch) = nested_switches.get(case_value)
        {
            // Add nested switch discriminator and case values to signature
            let nested_sig = format!(
                "__nested_{}_{:?}",
                nested_switch.switch_field,
                nested_switch.variant_fields.keys().collect::<Vec<_>>()
            );
            field_sig.push_str(&nested_sig);
        }

        field_groups
            .entry(field_sig)
            .or_insert_with(|| (*case_value, Vec::new()))
            .1
            .push(*case_value);
    }

    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    // Generate a reader for each variant struct
    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        // Get case fields, or use empty vec if this case only has a nested switch
        let case_fields = variant_fields
            .get(&first_value)
            .cloned()
            .unwrap_or_default();
        let variant_struct_name =
            super::enum_generation::generate_variant_struct_name(type_name, first_value);
        out.push_str(&generate_variant_struct_reader_impl(
            ctx,
            type_name,
            &variant_struct_name,
            field_set,
            &case_fields,
            first_value,
        ));
    }

    // Generate a reader for the main enum that delegates to variant structs
    out.push_str(&generate_enum_reader_impl(
        ctx,
        type_name,
        field_set,
        variant_fields,
    ));

    out
}

/// Generate a reader for the main enum that delegates to variant struct readers
fn generate_enum_reader_impl(
    ctx: &super::context::ReaderContext,
    type_name: &str,
    field_set: &FieldSet,
    variant_fields: &BTreeMap<i64, Vec<Field>>,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {type_name} {{\n"));
    out.push_str(
        "    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n",
    );

    // Read all common fields (these come before the switch)
    for field in &field_set.common_fields {
        let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
        let read_call = super::helpers::generate_read_call(ctx, field, &field_set.common_fields);

        // Alignment fields don't need to be stored, just executed
        if field.name.starts_with("__alignment_marker_") {
            out.push_str(&format!("        {}?;\n", read_call));
        } else {
            let allow_directive = get_allow_unused_directive(type_name, &field_name);
            out.push_str(allow_directive);
            out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
        }

        // Generate subfield computations if any
        for subfield in &field.subfields {
            let subfield_name = safe_identifier(&subfield.name, IdentifierType::Field).name;
            let allow_directive = get_allow_unused_directive(type_name, &subfield_name);
            let subfield_expr = super::helpers::convert_condition_expression(
                &subfield.value_expression,
                &field_set.common_fields,
            );
            let subfield_rust_type = get_rust_type(&subfield.field_type);
            out.push_str(allow_directive);
            out.push_str(&format!(
                "        let {} = ({}) as {};\n",
                subfield_name, subfield_expr, subfield_rust_type
            ));
        }
    }

    // Generate match on switch field
    let switch_field = field_set.switch_field.as_ref().unwrap();
    let switch_field_name = safe_identifier(switch_field, IdentifierType::Field).name;
    out.push_str(&format!("\n        match {} {{\n", switch_field_name));

    // Get the type of the switch field to see if it's an enum
    let switch_field_type = field_set
        .common_fields
        .iter()
        .find(|f| f.name == *switch_field)
        .map(|f| &f.field_type);

    // Collect all case values from both variant_fields and nested_switches
    let mut all_case_values = std::collections::BTreeSet::new();
    for case_value in variant_fields.keys() {
        all_case_values.insert(*case_value);
    }
    if let Some(ref nested_switches) = field_set.nested_switches {
        for case_value in nested_switches.keys() {
            all_case_values.insert(*case_value);
        }
    }

    // Group case values by field signature
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();

    for case_value in &all_case_values {
        let case_fields = variant_fields.get(case_value).cloned().unwrap_or_default();
        let mut field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        // Include nested switch structure in signature to avoid grouping cases with different nested switches
        if let Some(ref nested_switches) = field_set.nested_switches
            && let Some(nested_switch) = nested_switches.get(case_value)
        {
            // Add nested switch discriminator and case values to signature
            let nested_sig = format!(
                "__nested_{}_{:?}",
                nested_switch.switch_field,
                nested_switch.variant_fields.keys().collect::<Vec<_>>()
            );
            field_sig.push_str(&nested_sig);
        }

        field_groups
            .entry(field_sig)
            .or_insert_with(|| (*case_value, Vec::new()))
            .1
            .push(*case_value);
    }

    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        // Generate match patterns
        let mut case_patterns = Vec::new();
        for case_value in &sorted_values {
            // Convert case value to enum variant pattern if switch field is an enum
            let case_pattern = if let Some(switch_type) = switch_field_type {
                if ctx.enum_parent_map.contains_key(switch_type) {
                    // It's an enum - look up the variant name
                    if let Some(variant_name) =
                        ctx.enum_value_map.get(&(switch_type.clone(), *case_value))
                    {
                        // Use the enum variant: EnumType::VariantName
                        format!("{}::{}", switch_type, variant_name)
                    } else {
                        // Fallback to formatted hex value if variant not found
                        format_hex_value(*case_value)
                    }
                } else {
                    format_hex_value(*case_value)
                }
            } else {
                format_hex_value(*case_value)
            };
            case_patterns.push(case_pattern);
        }

        out.push_str(&format!(
            "            {} => {{\n",
            case_patterns.join(" | ")
        ));

        let variant_struct_name =
            super::enum_generation::generate_variant_struct_name(type_name, first_value);

        // Generate variant name
        let variant_name = if first_value < 0 {
            format!("TypeNeg{}", first_value.abs())
        } else {
            let hex_str = format!("{:X}", first_value);
            format!("Type{}", hex_str)
        };

        // Build the common fields argument list for the variant struct reader
        // Exclude the switch field since it's already been read and matched
        // Also exclude the nested switch field if this case has one
        let mut common_field_args = Vec::new();
        let switch_field = field_set.switch_field.as_ref().unwrap();
        let nested_switch_field = if let Some(ref nested_switches) = field_set.nested_switches {
            nested_switches
                .get(&first_value)
                .map(|ns| ns.switch_field.clone())
        } else {
            None
        };
        for field in &field_set.common_fields {
            let skip_field =
                field.name == *switch_field || (nested_switch_field.as_ref() == Some(&field.name));
            if !skip_field {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                common_field_args.push(field_name);
            }
        }
        let args_str = if common_field_args.is_empty() {
            String::new()
        } else {
            format!(", {}", common_field_args.join(", "))
        };

        out.push_str(&format!(
            "                let variant_struct = {variant_struct_name}::read(reader{})?;\n",
            args_str
        ));
        out.push_str(&format!(
            "                Ok(Self::{variant_name}(variant_struct))\n"
        ));
        out.push_str("            },\n");
    }

    // Check if we need a wildcard pattern
    // If the switch field is an enum and all variants are covered, we don't need it
    let need_wildcard = if let Some(switch_type) = switch_field_type {
        if ctx.enum_parent_map.contains_key(switch_type) {
            // It's an enum - check if all enum variants are covered
            let enum_variants: std::collections::BTreeSet<i64> = ctx
                .enum_value_map
                .iter()
                .filter_map(|((enum_name, value), _variant_name)| {
                    if enum_name == switch_type {
                        Some(*value)
                    } else {
                        None
                    }
                })
                .collect();

            // If all enum variants are in our case values, we don't need wildcard
            !enum_variants.is_subset(&all_case_values)
        } else {
            // Not an enum, need wildcard
            true
        }
    } else {
        // No type info, need wildcard
        true
    };

    if need_wildcard {
        out.push_str(&format!(
            "            _ => Err(format!(\"Unknown {{}} value: {{:?}}\", \"{switch_field_name}\", {switch_field_name}).into()),\n"
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation
    out.push_str(&format!(
        "impl crate::readers::ACDataType for {type_name} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {type_name}::read(reader)\n    }}\n}}\n\n"
    ));

    out
}

/// Generate a reader for a single variant struct
fn generate_variant_struct_reader_impl(
    ctx: &super::context::ReaderContext,
    type_name: &str,
    struct_name: &str,
    field_set: &FieldSet,
    case_fields: &[Field],
    case_value: i64,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {struct_name} {{\n"));

    // Check if this case has a nested switch and get its field name early
    let has_nested_switch = if let Some(ref nested_switches) = field_set.nested_switches {
        nested_switches.contains_key(&case_value)
    } else {
        false
    };

    // Get the nested switch field name if exists (as Option<String> for easier comparisons)
    let nested_switch_field_name = if has_nested_switch {
        field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .map(|ns| ns.switch_field.clone())
    } else {
        None
    };

    // Build function signature with common fields as parameters
    let mut params = vec!["reader: &mut dyn ACReader".to_string()];
    let switch_field = field_set.switch_field.as_ref().unwrap();
    for field in &field_set.common_fields {
        // Skip both the outer switch field and the nested switch field
        let skip_field =
            field.name == *switch_field || (nested_switch_field_name.as_ref() == Some(&field.name));
        if !skip_field {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            let field_type = &field.field_type;
            params.push(format!("{}: {}", field_name, field_type));
        }
    }
    let params_str = params.join(", ");

    out.push_str(&format!(
        "    pub fn read({}) -> Result<Self, Box<dyn std::error::Error>> {{\n",
        params_str
    ));

    // Don't read common fields - they're passed in as parameters

    // Read variant-specific fields
    for field in case_fields {
        // Skip the nested switch discriminator field
        let skip_field = nested_switch_field_name.as_ref() == Some(&field.name);
        if !skip_field {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            let mut all_fields = field_set.common_fields.clone();
            all_fields.extend(case_fields.iter().cloned());
            let read_call = super::helpers::generate_read_call(ctx, field, &all_fields);
            let allow_directive = get_allow_unused_directive(type_name, &field_name);
            out.push_str(allow_directive);
            out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
        }
    }

    if has_nested_switch {
        let nested_switch = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();

        // Read common fields before nested switch
        for field in &nested_switch.common_fields {
            if field.name != nested_switch.switch_field {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                let mut all_fields = field_set.common_fields.clone();
                all_fields.extend(case_fields.iter().cloned());
                let read_call = super::helpers::generate_read_call(ctx, field, &all_fields);
                let allow_directive = get_allow_unused_directive(type_name, &field_name);
                out.push_str(allow_directive);
                out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
            }
        }

        // Read the nested switch enum
        let nested_enum_name_raw =
            format!("{struct_name}{}{}", nested_switch.switch_field, "Variant");
        let nested_enum_name = safe_identifier(&nested_enum_name_raw, IdentifierType::Type).name;
        let nested_enum_field_name =
            safe_identifier(&nested_switch.switch_field, IdentifierType::Field).name;
        out.push_str(&format!(
            "        let {} = {}::read(reader)?;\n",
            nested_enum_field_name, nested_enum_name
        ));

        // Read trailing fields
        for field in &nested_switch.trailing_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            let mut all_fields = field_set.common_fields.clone();
            all_fields.extend(case_fields.iter().cloned());
            let read_call = super::helpers::generate_read_call(ctx, field, &all_fields);
            let allow_directive = get_allow_unused_directive(type_name, &field_name);
            out.push_str(allow_directive);
            out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
        }
    }

    // Construct the struct
    out.push_str("\n        Ok(Self {\n");

    for field in &field_set.common_fields {
        // Skip the outer switch field, nested switch field, and alignment marker fields
        let skip_field = field.name == *switch_field
            || (nested_switch_field_name.as_ref() == Some(&field.name))
            || field.name.starts_with("__alignment_marker_");
        if !skip_field {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("            {},\n", field_name));
        }
    }

    for field in case_fields {
        // Skip nested switch field and alignment marker fields
        let skip_field = (nested_switch_field_name.as_ref() == Some(&field.name))
            || field.name.starts_with("__alignment_marker_");
        if !skip_field {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("            {},\n", field_name));
        }
    }

    if has_nested_switch {
        let nested_switch = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();

        for field in &nested_switch.common_fields {
            if field.name != nested_switch.switch_field {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("            {},\n", field_name));
            }
        }

        let nested_field_name =
            safe_identifier(&nested_switch.switch_field, IdentifierType::Field).name;
        out.push_str(&format!("            {},\n", nested_field_name));

        for field in &nested_switch.trailing_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("            {},\n", field_name));
        }
    }

    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Don't generate ACDataType for variant structs since they require common field parameters
    // They're only called directly from their parent enum reader

    // Generate nested enum reader if this variant has a nested switch
    if has_nested_switch {
        let nested_switch = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();
        let nested_enum_name_raw =
            format!("{struct_name}{}{}", nested_switch.switch_field, "Variant");
        let nested_enum_name = safe_identifier(&nested_enum_name_raw, IdentifierType::Type).name;

        out.push_str(&generate_nested_switch_enum_reader(
            ctx,
            type_name,
            &nested_enum_name,
            nested_switch,
        ));
        out.push('\n');
    }

    out
}

/// Generate a reader for a nested switch enum
fn generate_nested_switch_enum_reader(
    ctx: &super::context::ReaderContext,
    type_name: &str,
    enum_name: &str,
    nested_switch: &NestedSwitch,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {enum_name} {{\n"));
    out.push_str(
        "    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n",
    );

    // Read the switch field
    let switch_field_name =
        safe_identifier(&nested_switch.switch_field, IdentifierType::Field).name;
    out.push_str(&format!(
        "        let {switch_field_name} = read_u8(reader)?;\n"
    ));

    out.push_str(&format!("\n        match {switch_field_name} {{\n"));

    // Group nested case values by field signature
    let mut field_groups: BTreeMap<String, (i64, Vec<i64>)> = BTreeMap::new();
    for (case_value, case_fields) in &nested_switch.variant_fields {
        let field_sig = case_fields
            .iter()
            .map(|f| format!("{}:{}", f.name, f.field_type))
            .collect::<Vec<_>>()
            .join(";");

        field_groups
            .entry(field_sig)
            .or_insert_with(|| (*case_value, Vec::new()))
            .1
            .push(*case_value);
    }

    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    for (_field_sig, (_primary_value, all_values)) in sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();

        let first_value = sorted_values[0];
        let case_fields = nested_switch
            .variant_fields
            .get(&first_value)
            .expect("Field group must have fields");

        // Generate match patterns for all values in group
        let mut case_patterns = Vec::new();
        for case_value in &sorted_values {
            case_patterns.push(format!("0x{:02X}", case_value));
        }

        out.push_str(&format!(
            "            {} => {{\n",
            case_patterns.join(" | ")
        ));

        // Read case fields
        for field in case_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            let read_call = super::helpers::generate_read_call(ctx, field, case_fields);
            let allow_directive = get_allow_unused_directive(type_name, &field_name);
            out.push_str(allow_directive);
            out.push_str(&format!(
                "                let {} = {}?;\n",
                field_name, read_call
            ));
        }

        // Generate variant name
        let variant_name = if first_value < 0 {
            format!("TypeNeg{}", first_value.abs())
        } else {
            format!("Type{:X}", first_value)
        };

        let struct_name = format!("{}{}", enum_name, variant_name);

        out.push_str(&format!(
            "                Ok(Self::{}({} {{\n",
            variant_name, struct_name
        ));
        for field in case_fields {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("                    {},\n", field_name));
        }
        out.push_str("                }))\n");
        out.push_str("            },\n");
    }

    out.push_str("            _ => Err(\"Unknown nested switch value\".into()),\n");
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out
}

/// Generate a reader for a simple struct (no variants)
fn generate_struct_reader_impl(
    ctx: &super::context::ReaderContext,
    _protocol_type: &ProtocolType,
    type_name: &str,
    field_set: &FieldSet,
) -> String {
    let mut out = String::new();

    out.push_str(&format!("impl {} {{\n", type_name));
    out.push_str(
        "    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {\n",
    );

    // Group consecutive fields with the same condition
    let field_groups =
        super::helpers::group_consecutive_fields_by_condition(&field_set.common_fields);

    // Generate reads for each group
    for group in &field_groups {
        super::helpers::generate_field_group_reads(
            ctx,
            type_name,
            &mut out,
            &group.condition,
            &group.fields,
            &field_set.common_fields,
        );
    }

    // Construct the struct
    out.push_str("\n        Ok(Self {\n");
    for field in &field_set.common_fields {
        // Skip alignment marker fields - they're only read, not stored
        if !field.name.starts_with("__alignment_marker_") {
            let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
            out.push_str(&format!("            {},\n", field_name));
        }
    }
    out.push_str("        })\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation
    out.push_str(&format!(
        "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        {}::read(reader)\n    }}\n}}\n\n",
        type_name, type_name
    ));

    out
}
