use std::collections::BTreeMap;

use crate::{
    field_gen::{
        DEFAULT_ENUM_DERIVES, build_derive_string, generate_field_line, get_allow_unused_directive,
    },
    identifiers::{IdentifierType, safe_enum_variant_name, safe_identifier, to_snake_case},
    type_utils::{get_rust_type, should_be_newtype_struct},
    types::{
        Field, FieldSet, IfBranch, NestedSwitch, ProtocolCategory, ProtocolEnum, ProtocolType,
    },
    util::format_hex_value,
};

/// Context for code generation, controlling what gets generated
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// List of type names to generate readers for
    /// Empty list means no readers
    /// ["*"] means all readers
    filter_types: Vec<String>,
}

impl GenerationContext {
    pub fn new(filter_types: Vec<String>) -> Self {
        Self { filter_types }
    }

    /// Check if readers should be generated for the given type name
    pub fn should_generate_reader(&self, type_name: &str) -> bool {
        if self.filter_types.is_empty() {
            return false;
        }

        if self.filter_types.contains(&"*".to_string()) {
            return true;
        }

        self.filter_types.contains(&type_name.to_string())
    }
}

/// Context for reader generation containing type information
pub struct ReaderContext {
    /// Map from enum name to its parent type (e.g., "NetAuthType" -> "uint")
    enum_parent_map: BTreeMap<String, String>,
    /// Map from (enum_name, value) to variant name (e.g., ("NetAuthType", 2) -> "AccountPassword")
    enum_value_map: BTreeMap<(String, i64), String>,
    /// Set of enum names that are bitflags (mask="true")
    mask_enums: std::collections::HashSet<String>,
}

impl ReaderContext {
    pub fn new(
        enum_parent_map: BTreeMap<String, String>,
        enum_value_map: BTreeMap<(String, i64), String>,
        mask_enums: std::collections::HashSet<String>,
    ) -> Self {
        Self {
            enum_parent_map,
            enum_value_map,
            mask_enums,
        }
    }
}

/// Build derive macro string from default derives plus extra derives

fn generate_bitflags(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // Get the underlying type for the bitflags
    let repr_type = if !protocol_enum.parent.is_empty() {
        get_rust_type(&protocol_enum.parent)
    } else {
        "u32" // Default to u32 if no parent specified
    };

    // Generate bitflags! macro invocation
    out.push_str("bitflags::bitflags! {\n");
    out.push_str("    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]\n");
    out.push_str(&format!("    pub struct {}: {} {{\n", enum_name, repr_type));

    for enum_value in &protocol_enum.values {
        let original_name = &enum_value.name;

        // Generate constant name - convert to SCREAMING_SNAKE_CASE for bitflags constants
        let const_name = if let Some(stripped) = original_name.strip_prefix("0x") {
            format!("TYPE_{}", stripped.to_uppercase())
        } else {
            // Convert from PascalCase to SCREAMING_SNAKE_CASE
            let mut result = String::new();
            let mut prev_was_lower = false;
            for (i, ch) in original_name.chars().enumerate() {
                if ch.is_uppercase() && i > 0 && prev_was_lower {
                    result.push('_');
                }
                result.push(ch.to_ascii_uppercase());
                prev_was_lower = ch.is_lowercase();
            }
            result
        };

        // Format the value as hex literal
        let value_literal = if enum_value.value < 0 {
            format!("{}", enum_value.value)
        } else {
            format!("0x{:X}", enum_value.value)
        };

        out.push_str(&format!(
            "        const {} = {};\n",
            const_name, value_literal
        ));
    }

    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Add ACDataType implementation for bitflags
    if !protocol_enum.parent.is_empty() {
        let read_fn = match repr_type {
            "u8" => "read_u8",
            "i8" => "read_i8",
            "u16" => "read_u16",
            "i16" => "read_i16",
            "u32" => "read_u32",
            "i32" => "read_i32",
            "u64" => "read_u64",
            "i64" => "read_i64",
            _ => "",
        };
        if !read_fn.is_empty() {
            out.push_str(&format!(
                "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        let value = crate::readers::{read_fn}(reader)?;\n        Ok({}::from_bits_retain(value))\n    }}\n}}\n\n",
                enum_name, enum_name
            ));
        }
    }

    out
}

fn generate_enum(protocol_enum: &ProtocolEnum) -> String {
    let enum_name = &protocol_enum.name;
    let mut out = String::new();

    if let Some(text_str) = &protocol_enum.text {
        out.push_str(&format!("/// {text_str}\n"));
    }

    // Generate all enums as regular enums (including mask enums)
    // Use enum-specific derives that include TryFromPrimitive
    let mut all_derives: Vec<String> = DEFAULT_ENUM_DERIVES.iter().map(|s| s.to_string()).collect();
    all_derives.extend(protocol_enum.extra_derives.iter().cloned());
    let derives = format!("#[derive({})]", all_derives.join(", "));

    // Add repr if parent is specified
    let repr_attr = if !protocol_enum.parent.is_empty() {
        let repr_type = get_rust_type(&protocol_enum.parent);
        format!("#[repr({repr_type})]\n")
    } else {
        String::new()
    };

    out.push_str(&format!("{repr_attr}{derives}\npub enum "));
    out.push_str(enum_name);
    out.push_str(" {\n");

    for enum_value in &protocol_enum.values {
        let original_name = &enum_value.name;

        // Generate variant name - convert to PascalCase if it has underscores
        let variant_name = if let Some(stripped) = original_name.strip_prefix("0x") {
            format!("Type{stripped}")
        } else {
            original_name.clone()
        };

        // Check if variant name is a reserved word or needs conversion
        let safe_variant = safe_enum_variant_name(&variant_name);

        // Determine if we need serde rename (if the safe name differs from original)
        let needs_serde_rename = safe_variant.name != *original_name;

        // Format the value as hex literal for enum definition
        let value_literal = if enum_value.value < 0 {
            // Negative values use decimal
            format!("{}", enum_value.value)
        } else {
            // Positive values use hex literal (natural width, no padding)
            format!("0x{:X}", enum_value.value)
        };

        if needs_serde_rename {
            out.push_str(&format!(
                "    #[serde(rename = \"{}\")]\n    {} = {},\n",
                original_name, safe_variant.name, value_literal
            ));
        } else {
            out.push_str(&format!("    {} = {},\n", safe_variant.name, value_literal));
        }
    }

    out.push_str("}\n\n");

    // Add ACDataType implementation for enums
    if !protocol_enum.parent.is_empty() {
        let parent_rust = get_rust_type(&protocol_enum.parent);
        let read_fn = match parent_rust {
            "u8" => "read_u8",
            "i8" => "read_i8",
            "u16" => "read_u16",
            "i16" => "read_i16",
            "u32" => "read_u32",
            "i32" => "read_i32",
            "u64" => "read_u64",
            "i64" => "read_i64",
            _ => "",
        };
        if !read_fn.is_empty() {
            out.push_str(&format!(
                "impl crate::readers::ACDataType for {} {{\n    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {{\n        let value = crate::readers::{read_fn}(reader)?;\n        Ok({}::try_from(value)?)\n    }}\n}}\n\n",
                enum_name, enum_name
            ));
        }
    }

    out
}

/// Analyze all types and add extra derives where needed
fn rectify_dependencies(
    common_types: &[ProtocolType],
    c2s_types: &[ProtocolType],
    s2c_types: &[ProtocolType],
    enums: &mut [ProtocolEnum],
    common_types_out: &mut Vec<ProtocolType>,
    c2s_types_out: &mut Vec<ProtocolType>,
    s2c_types_out: &mut Vec<ProtocolType>,
) {
    let mut hash_types = std::collections::HashSet::new();

    // Collect all type names that need Hash from all type collections
    for protocol_type in common_types.iter().chain(c2s_types).chain(s2c_types) {
        extract_hash_requirements_from_type(protocol_type, &mut hash_types);
    }

    // Add Hash+Eq to enums that need them
    for protocol_enum in enums.iter_mut() {
        if hash_types.contains(&protocol_enum.name) {
            protocol_enum.extra_derives.push("Hash".to_string());
            protocol_enum.extra_derives.push("Eq".to_string());
        }
    }

    // Add Hash+Eq to types that need them and copy to output vectors
    for protocol_type in common_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        common_types_out.push(updated_type);
    }

    for protocol_type in c2s_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        c2s_types_out.push(updated_type);
    }

    for protocol_type in s2c_types {
        let mut updated_type = protocol_type.clone();
        if hash_types.contains(&protocol_type.name) {
            updated_type.extra_derives.push("Hash".to_string());
            updated_type.extra_derives.push("Eq".to_string());
        }
        s2c_types_out.push(updated_type);
    }
}

/// Extract type names that need Hash from a single ProtocolType
fn extract_hash_requirements_from_type(
    protocol_type: &ProtocolType,
    hash_types: &mut std::collections::HashSet<String>,
) {
    if let Some(ref field_set) = protocol_type.fields {
        // Check common fields for HashMap usage
        for field in &field_set.common_fields {
            extract_hash_requirements_from_field(&field.field_type, hash_types);
        }

        // Check variant fields for HashMap usage
        if let Some(ref variant_fields) = field_set.variant_fields {
            for case_fields in variant_fields.values() {
                for field in case_fields {
                    extract_hash_requirements_from_field(&field.field_type, hash_types);
                }
            }
        }
    }
}

/// Extract type names that need Hash from a field type string
fn extract_hash_requirements_from_field(
    field_type: &str,
    hash_types: &mut std::collections::HashSet<String>,
) {
    // Look for HashMap<KeyType, V> patterns
    if field_type.starts_with("std::collections::HashMap<") {
        let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
        if let Some(comma_pos) = inner.find(',') {
            let key_type = inner[..comma_pos].trim();
            // Check if the key type is not a single letter generic (like T, U)
            if key_type.len() > 1 && key_type.chars().next().unwrap().is_ascii_uppercase() {
                hash_types.insert(key_type.to_string());
            }
        }
    }

    // Also check for PackableHashTable<KeyType, V> patterns
    if field_type.starts_with("PackableHashTable<") {
        let inner = &field_type["PackableHashTable<".len()..field_type.len() - 1];
        if let Some(comma_pos) = inner.find(',') {
            let key_type = inner[..comma_pos].trim();
            if key_type.len() > 1 && key_type.chars().next().unwrap().is_ascii_uppercase() {
                hash_types.insert(key_type.to_string());
            }
        }
    }
}

/// Generate a name for a variant struct
fn generate_variant_struct_name(parent_type_name: &str, case_value: i64) -> String {
    let case_hex_str = if case_value < 0 {
        format!("Neg{}", case_value.abs())
    } else {
        format!("{:X}", case_value)
    };
    let name = format!("{parent_type_name}Type{case_hex_str}");
    // Apply safe_identifier to ensure proper Rust PascalCase naming
    let safe_name = safe_identifier(&name, IdentifierType::Type);
    safe_name.name
}

/// Generate a standalone struct for a single enum variant with a nested switch
fn generate_variant_struct(
    parent_type_name: &str,
    case_value: i64,
    field_set: &FieldSet,
    switch_field: &str,
    case_fields: &[Field],
) -> String {
    let struct_name = generate_variant_struct_name(parent_type_name, case_value);
    let mut out = String::new();

    let derives = build_derive_string(&[]);
    out.push_str(&format!("{derives}\n"));
    out.push_str(&format!("pub struct {struct_name} {{\n"));

    // Check if this case has a nested switch
    let has_nested_switch = if let Some(ref nested_switches) = field_set.nested_switches {
        nested_switches.contains_key(&case_value)
    } else {
        false
    };

    // Get the nested switch field name to skip it in common fields
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

    // Add common fields first (excluding the switch fields)
    for field in &field_set.common_fields {
        // Skip the outer switch field, nested switch field, and alignment marker fields
        if field.name != switch_field
            && Some(field.name.clone()) != nested_switch_field_name
            && !field.name.starts_with("__alignment_marker_")
        {
            out.push_str(&generate_field_line(field, false)); // false = struct field
            out.push_str(",\n");
        }
    }

    // Add variant-specific fields before the nested switch
    // We already have nested_switch_field_name from above
    for field in case_fields {
        // Skip the nested switch discriminator field - it will be represented by the enum
        // Also skip alignment marker fields - they're only for reading
        let skip_field = nested_switch_field_name.as_ref() == Some(&field.name);
        if !skip_field && !field.name.starts_with("__alignment_marker_") {
            out.push_str(&generate_field_line(field, false));
            out.push_str(",\n");
        }
    }

    if has_nested_switch {
        let nested_switch_obj = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();

        // Add common fields for the nested switch (fields that come after case fields but before the switch cases)
        // Skip the switch field itself since it becomes the nested enum
        for field in &nested_switch_obj.common_fields {
            if field.name != nested_switch_obj.switch_field {
                out.push_str(&generate_field_line(field, false));
                out.push_str(",\n");
            }
        }

        // Generate a nested enum for the switch
        let nested_enum_name_raw = format!(
            "{struct_name}{}{}",
            nested_switch_obj.switch_field, "Variant"
        );
        let nested_enum_name = safe_identifier(&nested_enum_name_raw, IdentifierType::Type).name;
        out.push_str(&format!(
            "    pub {}: {nested_enum_name},\n",
            safe_identifier(&nested_switch_obj.switch_field, IdentifierType::Field).name
        ));

        // Add trailing fields (fields after the nested switch within the case)
        for field in &nested_switch_obj.trailing_fields {
            out.push_str(&generate_field_line(field, false));
            out.push_str(",\n");
        }
    }

    out.push_str("}\n\n");

    // If there's a nested switch, generate the nested enum
    if has_nested_switch {
        let nested_switch_obj = field_set
            .nested_switches
            .as_ref()
            .unwrap()
            .get(&case_value)
            .unwrap();
        let nested_enum_name_raw = format!(
            "{struct_name}{}{}",
            nested_switch_obj.switch_field, "Variant"
        );
        let nested_enum_name = safe_identifier(&nested_enum_name_raw, IdentifierType::Type).name;

        out.push_str(&generate_nested_switch_enum(
            &nested_enum_name,
            nested_switch_obj,
        ));
    }

    out
}

/// Generate an enum for a nested switch
fn generate_nested_switch_enum(enum_name: &str, nested_switch: &NestedSwitch) -> String {
    let mut out = String::new();

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

    // Sort by primary value for consistent output
    let mut sorted_groups: Vec<_> = field_groups.into_iter().collect();
    sorted_groups.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    // First, generate standalone variant structs
    for (_field_sig, (_primary_value, all_values)) in &sorted_groups {
        let mut sorted_values = all_values.clone();
        sorted_values.sort();
        let first_value = sorted_values[0];

        let variant_name = if first_value < 0 {
            format!("TypeNeg{}", first_value.abs())
        } else {
            let hex_str = format!("{:X}", first_value);
            format!("Type{}", hex_str)
        };

        let struct_name = format!("{}{}", enum_name, variant_name);

        let derives = build_derive_string(&[]);
        out.push_str(&format!("{derives}\n"));
        out.push_str(&format!("pub struct {struct_name} {{\n"));

        if let Some(case_fields) = nested_switch.variant_fields.get(&first_value) {
            for field in case_fields {
                out.push_str(&generate_field_line(field, false)); // false = is struct field
                out.push_str(",\n");
            }
        }

        out.push_str("}\n\n");
    }

    // Now generate the enum that references the standalone structs
    let derives = build_derive_string(&[]);
    out.push_str(&format!("{derives}\n"));
    out.push_str("#[serde(tag = \"");
    out.push_str(&nested_switch.switch_field);
    out.push_str("\")]\n");
    out.push_str(&format!("pub enum {enum_name} {{\n"));

    for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
        all_values.sort();

        let first_value = all_values[0];
        let first_value_str = format_hex_value(first_value);

        let variant_name = if first_value < 0 {
            format!("TypeNeg{}", first_value.abs())
        } else {
            let hex_str = format!("{:X}", first_value);
            format!("Type{}", hex_str)
        };

        let struct_name = format!("{}{}", enum_name, variant_name);

        out.push_str(&format!("    #[serde(rename = \"{first_value_str}\")]\n"));

        for alias_value in &all_values[1..] {
            let alias_str = format_hex_value(*alias_value);
            out.push_str(&format!("    #[serde(alias = \"{alias_str}\")]\n"));
        }

        out.push_str(&format!("    {variant_name}({struct_name}),\n"));
    }

    out.push_str("}\n\n");
    out
}

fn generate_type(protocol_type: &ProtocolType) -> String {
    let original_type_name = &protocol_type.name;
    println!("generate_type: name = {original_type_name}");

    // Convert type name to PascalCase
    let safe_type_name = safe_identifier(original_type_name, IdentifierType::Type);
    let type_name = &safe_type_name.name;

    let mut out = String::new();

    if let Some(text_str) = &protocol_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    // Handle templated types - we'll generate them as generic structs
    // Continue processing to include fields
    let type_generics = if let Some(template_params) = &protocol_type.templated {
        if protocol_type.hash_bounds.is_empty() {
            format!("<{template_params}>")
        } else {
            // Add Hash + Eq bounds to generic parameters that need them
            let params: Vec<&str> = template_params.split(',').map(|s| s.trim()).collect();
            let bounded_params: Vec<String> = params
                .iter()
                .map(|param| {
                    if protocol_type.hash_bounds.contains(&param.to_string()) {
                        format!("{param}: std::cmp::Eq + std::hash::Hash")
                    } else {
                        param.to_string()
                    }
                })
                .collect();
            format!("<{}>", bounded_params.join(", "))
        }
    } else {
        String::new()
    };

    // Handle non-templated parent types as type aliases or newtype structs
    // Note: protocol.xml quirk - some types have both parent and templated attributes
    // (e.g., PackableList has parent="List" and templated="T")
    // For templated types, skip the parent alias and continue to struct generation
    if protocol_type.templated.is_none()
        && let Some(parent_type) = &protocol_type.parent
    {
        let rust_type = get_rust_type(parent_type);

        // Only generate if the rust type differs from the XML type name
        if rust_type != *original_type_name {
            if should_be_newtype_struct(type_name, rust_type) {
                // Generate newtype struct for semantic types
                // WString wraps String (not Copy), others wrap primitives (Copy)
                if type_name == "WString" {
                    out.push_str("#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]\n");
                } else {
                    out.push_str(
                        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]\n",
                    );
                }
                out.push_str("#[derive(serde::Serialize, serde::Deserialize)]\n");
                out.push_str("#[serde(transparent)]\n");
                out.push_str(&format!("pub struct {type_name}(pub {rust_type});\n\n"));
            } else {
                // Generate type alias for C-style aliases or non-primitive parents
                out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        }
        return out;
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{}}\n\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\npub struct {type_name}{type_generics} {{}}\n\n"
            ));
        }
        return out;
    };

    // Check if this is a variant type (has switch) or simple type
    if let Some(ref variant_fields) = field_set.variant_fields {
        if type_name.contains("TurbineChat") {
            use std::fs::OpenOptions;
            use std::io::Write;
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("/tmp/turbine_debug.txt")
                .unwrap();
            writeln!(
                file,
                "DEBUG: Type {} has variant_fields with {} cases",
                type_name,
                variant_fields.len()
            )
            .unwrap();
            for (case_val, fields) in variant_fields {
                writeln!(file, "  Case {}: {} fields", case_val, fields.len()).unwrap();
            }
            writeln!(
                file,
                "DEBUG: Has nested_switches: {}",
                field_set.nested_switches.is_some()
            )
            .unwrap();
            if let Some(ref ns) = field_set.nested_switches {
                writeln!(file, "  Nested switches for {} cases", ns.len()).unwrap();
            }
        }
        // First, generate all standalone variant structs
        let switch_field = field_set.switch_field.as_ref().unwrap();

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

        // Group case values by their field sets (to handle multi-value cases)
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

        // Generate standalone variant structs for each variant
        for (_field_sig, (_primary_value, all_values)) in &sorted_groups {
            let mut sorted_values = all_values.clone();
            sorted_values.sort();
            let first_value = sorted_values[0];
            // Get case fields, or use empty vec if this case only has a nested switch
            let case_fields = variant_fields
                .get(&first_value)
                .cloned()
                .unwrap_or_default();
            out.push_str(&generate_variant_struct(
                type_name,
                first_value,
                field_set,
                switch_field,
                &case_fields,
            ));
        }

        // Now generate the main enum that references these structs
        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\n#[serde(tag = \"{switch_field}\")]
pub enum {type_name}{type_generics} {{\n"
            ));
        }

        for (_field_sig, (_primary_value, mut all_values)) in sorted_groups {
            all_values.sort();

            let first_value = all_values[0];
            let first_value_str = format_hex_value(first_value);

            let variant_name = if first_value < 0 {
                format!("TypeNeg{}", first_value.abs())
            } else {
                let hex_str = format!("{:X}", first_value);
                format!("Type{}", hex_str)
            };

            let variant_struct_name = generate_variant_struct_name(type_name, first_value);

            // Primary serde rename
            out.push_str(&format!("    #[serde(rename = \"{first_value_str}\")]\n"));

            // Add aliases for additional values (if multi-value case)
            for alias_value in &all_values[1..] {
                let alias_str = format_hex_value(*alias_value);
                out.push_str(&format!("    #[serde(alias = \"{alias_str}\")]\n"));
            }

            // Use tuple variant that wraps the standalone struct
            out.push_str(&format!("    {variant_name}({variant_struct_name}),\n"));
        }

        out.push_str("}\n\n");
    } else {
        // Generate struct
        let mut field_out: Vec<String> = Vec::new();

        for field in &field_set.common_fields {
            // Skip alignment marker fields - they're only for reading
            if !field.name.starts_with("__alignment_marker_") {
                field_out.push(generate_field_line(field, false)); // false = is struct field
            }
        }

        let fields_out: String = field_out.join(",\n") + ",";

        let derives = build_derive_string(&protocol_type.extra_derives);

        if safe_type_name.needs_rename {
            out.push_str(&format!(
                "{derives}\n#[serde(rename = \"{original_type_name}\")]
pub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
            ));
        } else {
            out.push_str(&format!(
                "{derives}\npub struct {type_name}{type_generics} {{
{fields_out}
}}\n\n"
            ));
        }
    }

    out
}

pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

pub struct GeneratedCode {
    pub files: Vec<GeneratedFile>,
}

/// Represents the top-level protocol section a type/enum comes from
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Indicates the source of the XML being parsed (protocol.xml vs network.xml)
pub enum GenerateSource {
    Protocol,
    Network,
}

// ProtocolCategory is defined in types.rs (moved there to avoid circular imports)

/// Generate a single file containing both type definition and reader implementation
fn generate_type_and_reader_file(
    gen_ctx: &GenerationContext,
    reader_ctx: &ReaderContext,
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
        out.push_str(&generate_type(protocol_type));
    }

    // Generate reader implementation if requested
    if gen_ctx.should_generate_reader(&protocol_type.name) {
        out.push_str(&generate_reader_impl(reader_ctx, protocol_type));
    }

    out
}

/// Generate a reader implementation for a single type (as an impl block on the type)
fn generate_reader_impl(ctx: &ReaderContext, protocol_type: &ProtocolType) -> String {
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

            if should_be_newtype_struct(&safe_type_name.name, rust_type) {
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
    ctx: &ReaderContext,
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
        let variant_struct_name = generate_variant_struct_name(type_name, first_value);
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
    ctx: &ReaderContext,
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
        let read_call = generate_read_call(ctx, field, &field_set.common_fields);

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
            let subfield_expr =
                convert_condition_expression(&subfield.value_expression, &field_set.common_fields);
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

        let variant_struct_name = generate_variant_struct_name(type_name, first_value);

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
    ctx: &ReaderContext,
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
            let read_call = generate_read_call(ctx, field, &all_fields);
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
                let read_call = generate_read_call(ctx, field, &all_fields);
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
            let read_call = generate_read_call(ctx, field, &all_fields);
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
    ctx: &ReaderContext,
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
            let read_call = generate_read_call(ctx, field, case_fields);
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
    ctx: &ReaderContext,
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
    let field_groups = group_consecutive_fields_by_condition(&field_set.common_fields);

    // Generate reads for each group
    for group in &field_groups {
        generate_field_group_reads(
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

/// A group of consecutive fields with the same condition
#[derive(Debug)]
struct FieldGroup<'a> {
    condition: ConditionKey,
    fields: Vec<&'a Field>,
}

/// Group consecutive fields with the same condition for coalescing conditional reads
/// This preserves field order from the XML
fn group_consecutive_fields_by_condition(fields: &[Field]) -> Vec<FieldGroup<'_>> {
    let mut groups = Vec::new();

    if fields.is_empty() {
        return groups;
    }

    let mut current_key = ConditionKey::from_field(&fields[0]);
    let mut current_group = vec![&fields[0]];

    for field in &fields[1..] {
        let key = ConditionKey::from_field(field);

        if key == current_key {
            // Same condition - add to current group
            current_group.push(field);
        } else {
            // Different condition - start new group
            groups.push(FieldGroup {
                condition: current_key,
                fields: current_group,
            });
            current_key = key;
            current_group = vec![field];
        }
    }

    // Don't forget the last group
    groups.push(FieldGroup {
        condition: current_key,
        fields: current_group,
    });

    groups
}

/// Key for grouping fields by their condition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ConditionKey {
    /// No condition - always read
    None,
    /// <if test="condition">
    IfTest(String),
    /// <maskmap> with specific field and mask value
    Mask { field: String, value: String },
}

impl ConditionKey {
    fn from_field(field: &Field) -> Self {
        if let Some(condition) = &field.optional_condition {
            ConditionKey::IfTest(condition.clone())
        } else if let (Some(mask_field), Some(mask_value)) = (&field.mask_field, &field.mask_value)
        {
            ConditionKey::Mask {
                field: mask_field.clone(),
                value: mask_value.clone(),
            }
        } else {
            ConditionKey::None
        }
    }
}

/// Generate code to assign a field that appears in both branches
///
/// Since the variable is pre-declared with `let field;`, we just assign to it.
/// This avoids the unused_assignments warning and is cleaner.
fn generate_both_branch_field_read_no_let(
    ctx: &ReaderContext,
    field: &Field,
    all_fields: &[Field],
    field_name: &str,
    out: &mut String,
) {
    // Check if this field has a different type in the false branch
    if let Some(false_type_str) = &field.if_false_branch_type {
        let rust_type_merged = get_rust_type(&field.field_type);
        let rust_type_false = get_rust_type(false_type_str);

        if rust_type_false != rust_type_merged {
            // Types differ - read the smaller type from false branch and cast to merged type
            let mut false_field_temp = (*field).clone();
            false_field_temp.field_type = false_type_str.clone();
            let false_read_call = generate_read_base_logic(ctx, &false_field_temp, all_fields);
            out.push_str(&format!(
                "            {} = Some(({})? as {});\n",
                field_name, false_read_call, rust_type_merged
            ));
        } else {
            // Types match - just read normally
            out.push_str(&format!(
                "            {} = Some({}?);\n",
                field_name,
                generate_read_base_logic(ctx, field, all_fields)
            ));
        }
    } else {
        // No type difference recorded, read the merged type normally
        out.push_str(&format!(
            "            {} = Some({}?);\n",
            field_name,
            generate_read_base_logic(ctx, field, all_fields)
        ));
    }
}

/// Generate mask field expression, casting to u32 if it's an enum type
fn generate_mask_field_expr(
    ctx: &ReaderContext,
    mask_field_name: &str,
    mask_field_safe: &str,
    all_fields: &[Field],
) -> String {
    if let Some(mask_field_obj) = all_fields.iter().find(|f| f.name == mask_field_name) {
        let base_expr = if ctx.enum_parent_map.contains_key(&mask_field_obj.field_type) {
            // It's an enum or bitflags - get bits representation
            if ctx.mask_enums.contains(&mask_field_obj.field_type) {
                // Bitflags - use .bits() method
                format!("{}.bits()", mask_field_safe)
            } else {
                // Regular enum - clone and cast to u32 (enums don't derive Copy)
                format!("{}.clone() as u32", mask_field_safe)
            }
        } else {
            mask_field_safe.to_string()
        };
        // If the mask field is optional, unwrap it with a default of 0
        if mask_field_obj.is_optional {
            format!("{}.unwrap_or(0)", base_expr)
        } else {
            base_expr
        }
    } else {
        mask_field_safe.to_string()
    }
}

/// Generate reads for a group of fields with the same condition
fn generate_field_group_reads(
    ctx: &ReaderContext,
    type_name: &str,
    out: &mut String,
    condition_key: &ConditionKey,
    fields: &[&Field],
    all_fields: &[Field],
) {
    match condition_key {
        ConditionKey::None => {
            // No condition - just read each field directly
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                let read_call = generate_read_base_logic(ctx, field, all_fields);
                let allow_directive = get_allow_unused_directive(type_name, &field_name);
                out.push_str(allow_directive);

                // Alignment fields don't need to be stored, just executed
                if field.name.starts_with("__alignment_marker_") {
                    out.push_str(&format!("        {}?;\n", read_call));
                } else {
                    out.push_str(&format!("        let {} = {}?;\n", field_name, read_call));
                }

                // Generate subfield computations if any
                for subfield in &field.subfields {
                    let subfield_name = safe_identifier(&subfield.name, IdentifierType::Field).name;
                    let allow_directive = get_allow_unused_directive(type_name, &subfield_name);
                    let subfield_expr =
                        convert_condition_expression(&subfield.value_expression, all_fields);
                    let subfield_rust_type = get_rust_type(&subfield.field_type);
                    out.push_str(allow_directive);
                    out.push_str(&format!(
                        "        let {} = ({}) as {};\n",
                        subfield_name, subfield_expr, subfield_rust_type
                    ));
                }
            }
        }
        ConditionKey::IfTest(condition) => {
            // Handle <if test="condition"> with <true> and <false> branches
            let rust_condition = convert_condition_expression(condition, all_fields);

            // Categorize fields by which branch(es) they belong to
            let mut true_only = Vec::new();
            let mut false_only = Vec::new();
            let mut both = Vec::new();

            for field in fields {
                match &field.if_branch {
                    Some(IfBranch::True) => true_only.push(*field),
                    Some(IfBranch::False) => false_only.push(*field),
                    Some(IfBranch::Both) => both.push(*field),
                    None => both.push(*field), // Fallback for fields without explicit branch
                }
            }

            // Declare fields that need to be available after the if-else block.
            // - true_only and false_only: initialized to None, may or may not be assigned
            // - both: will be assigned in both branches, declared with underscore init (never read)
            for field in true_only.iter().chain(false_only.iter()) {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }
            for field in &both {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let {};\n", field_name));
            }

            // Generate if-else block that assigns values to the variables
            out.push_str(&format!("        if {} {{\n", rust_condition));

            // TRUE branch: read all true-only and both-branch fields
            for field in &true_only {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name,
                    generate_read_base_logic(ctx, field, all_fields)
                ));
            }
            for field in &both {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name,
                    generate_read_base_logic(ctx, field, all_fields)
                ));
            }

            // FALSE branch: only emit if there are fields to read in the false branch
            if !false_only.is_empty() || !both.is_empty() {
                out.push_str("        } else {\n");

                // Read false-only fields
                for field in &false_only {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    out.push_str(&format!(
                        "            {} = Some({}?);\n",
                        field_name,
                        generate_read_base_logic(ctx, field, all_fields)
                    ));
                }

                // Read both-branch fields, with type casting if needed
                for field in &both {
                    let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    generate_both_branch_field_read_no_let(
                        ctx,
                        field,
                        all_fields,
                        &field_name,
                        &mut *out,
                    );
                }

                out.push_str("        }\n");
            } else {
                out.push_str("        }\n");
            }
        }
        ConditionKey::Mask {
            field: mask_field,
            value: mask_value,
        } => {
            // Generate a single if block for all fields with this mask
            let mask_field_safe = safe_identifier(mask_field, IdentifierType::Field).name;
            let mask_value_code = if mask_value.contains('.') {
                let parts: Vec<&str> = mask_value.split('.').collect();
                if parts.len() == 2 {
                    let enum_name = parts[0];
                    let variant_name = parts[1];
                    // Check if this is a mask enum (bitflags)
                    if ctx.mask_enums.contains(enum_name) {
                        // Convert variant name to SCREAMING_SNAKE_CASE for bitflags
                        let const_name = if let Some(stripped) = variant_name.strip_prefix("0x") {
                            format!("TYPE_{}", stripped.to_uppercase())
                        } else {
                            // Convert from PascalCase to SCREAMING_SNAKE_CASE
                            let mut result = String::new();
                            let mut prev_was_lower = false;
                            for (i, ch) in variant_name.chars().enumerate() {
                                if ch.is_uppercase() && i > 0 && prev_was_lower {
                                    result.push('_');
                                }
                                result.push(ch.to_ascii_uppercase());
                                prev_was_lower = ch.is_lowercase();
                            }
                            result
                        };
                        format!("{}::{}.bits()", enum_name, const_name)
                    } else {
                        // Regular enum - cast to u32
                        format!("{}::{} as u32", enum_name, variant_name)
                    }
                } else {
                    mask_value.clone()
                }
            } else {
                mask_value.clone()
            };

            // Initialize all optional fields to None first
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }

            // Generate the if block
            // Cast mask field to u32 if it's an enum type
            let mask_field_expr =
                generate_mask_field_expr(ctx, mask_field, &mask_field_safe, all_fields);

            out.push_str(&format!(
                "        if ({} & {}) != 0 {{\n",
                mask_field_expr, mask_value_code
            ));
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                let read_call = generate_read_base_logic(ctx, field, all_fields);
                out.push_str(&format!(
                    "            {} = Some({}?);\n",
                    field_name, read_call
                ));
            }
            out.push_str("        }\n");
        }
    }
}

/// Generate the base read call without the conditional wrapper
/// Helper function that generates base read call without optional field handling
fn generate_read_base_logic(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let field_type = &field.field_type;

    // Handle alignment padding fields first
    if let Some(align_type) = field_type.strip_prefix("__align__") {
        let padding_code = match align_type {
            "DWORD" => "align_dword(reader)",
            "WORD" => "align_word(reader)",
            "QWORD" => "align_qword(reader)",
            _ => "read_u32(reader)",
        };
        return padding_code.to_string();
    }

    let rust_type = get_rust_type(field_type);

    match rust_type {
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
        "String" => "read_string(reader)".to_string(),
        "WString" => "read_wstring(reader).map(WString)".to_string(),
        _ => {
            // Check if it's an enum
            if let Some(parent_type) = ctx.enum_parent_map.get(field_type) {
                // It's an enum - read the parent type and cast
                let parent_rust_type = get_rust_type(parent_type);
                let read_fn = match parent_rust_type {
                    "u8" => "read_u8",
                    "i8" => "read_i8",
                    "u16" => "read_u16",
                    "i16" => "read_i16",
                    "u32" => "read_u32",
                    "i32" => "read_i32",
                    "u64" => "read_u64",
                    "i64" => "read_i64",
                    _ => panic!("Unsupported enum parent type: {}", parent_type),
                };
                // Use from_bits_retain for bitflags types, try_from for regular enums
                if ctx.mask_enums.contains(field_type) {
                    format!(
                        "Ok::<_, Box<dyn std::error::Error>>({}::from_bits_retain({}(reader)?))",
                        field_type, read_fn
                    )
                } else {
                    format!("{}::try_from({}(reader)?)", field_type, read_fn)
                }
            } else if field_type.starts_with("Vec<") {
                // Handle Vec - extract element type
                let element_type = &field_type[4..field_type.len() - 1];

                if let Some(length_expr) = &field.length_expression {
                    // Convert length expression to Rust code
                    let length_code = convert_length_expression(length_expr, all_fields);
                    generate_vec_read_with_length(element_type, &length_code)
                } else {
                    "unimplemented!(\"Vec reading without length not yet implemented\")".to_string()
                }
            } else if field_type.starts_with("PackableList<") {
                let element_type = &field_type[13..field_type.len() - 1];
                generate_packable_list_read(element_type)
            } else if field_type.starts_with("std::collections::HashMap<") {
                // Handle HashMap - extract key and value types
                let inner = &field_type["std::collections::HashMap<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();

                    if let Some(length_expr) = &field.length_expression {
                        // Convert length expression to Rust code
                        let length_code = convert_length_expression(length_expr, all_fields);
                        generate_hashmap_read_with_length(key_type, value_type, &length_code, ctx)
                    } else {
                        "unimplemented!(\"HashMap reading without length not yet implemented\")"
                            .to_string()
                    }
                } else {
                    "unimplemented!(\"HashMap reading with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PackableHashTable<") {
                let inner = &field_type["PackableHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    generate_packable_hash_table_read(key_type, value_type, ctx)
                } else {
                    "unimplemented!(\"PackableHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else if field_type.starts_with("PHashTable<") {
                let inner = &field_type["PHashTable<".len()..field_type.len() - 1];
                if let Some(comma_pos) = inner.find(',') {
                    let key_type = inner[..comma_pos].trim();
                    let value_type = inner[comma_pos + 1..].trim();
                    generate_phash_table_read(key_type, value_type)
                } else {
                    "unimplemented!(\"PHashTable with invalid type not yet implemented\")"
                        .to_string()
                }
            } else {
                // Custom struct type - call its read method
                // If the type has generic parameters (contains '<'), we need turbofish syntax
                if let Some(pos) = field_type.find('<') {
                    let (type_name, generics) = field_type.split_at(pos);
                    format!("{type_name}::{generics}::read(reader)")
                } else {
                    format!("{field_type}::read(reader)")
                }
            }
        }
    }
}

/// Generate the appropriate read call for a given field
///
/// # Arguments
/// * `ctx` - Reader context with enum information
/// * `field` - The field to generate a read call for
/// * `all_fields` - All fields available in the current scope (used to resolve length expressions)
fn generate_read_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let base_read = generate_read_base_logic(ctx, field, all_fields);

    if field.is_optional {
        // Generate conditional read based on the test condition or mask
        if let Some(condition) = &field.optional_condition {
            // <if test="..."> condition
            // Convert the condition to Rust code (e.g., "RecordCount > 0" -> "record_count > 0")
            let rust_condition = convert_condition_expression(condition, all_fields);
            // The base_read returns Result<T, E>, we want Result<Option<T>, E>
            // If condition is true, read and wrap in Some; otherwise return Ok(None)
            format!(
                "if {} {{ {}.map(Some) }} else {{ Ok(None) }}",
                rust_condition, base_read
            )
        } else if let (Some(mask_field), Some(mask_value)) = (&field.mask_field, &field.mask_value)
        {
            // <maskmap> with <mask value="...">
            // Generate bitwise AND check
            let mask_field_safe = safe_identifier(mask_field, IdentifierType::Field).name;
            // The mask_value might be a plain hex number or an enum variant like "ACBaseQualitiesFlags.PropertyInt"
            let mask_value_code = if mask_value.contains('.') {
                // It's an enum variant reference, convert to Rust format
                let parts: Vec<&str> = mask_value.split('.').collect();
                if parts.len() == 2 {
                    let enum_name = parts[0];
                    let variant_name = parts[1];
                    // Check if this is a mask enum (bitflags)
                    if ctx.mask_enums.contains(enum_name) {
                        // Convert variant name to SCREAMING_SNAKE_CASE for bitflags
                        let const_name = if let Some(stripped) = variant_name.strip_prefix("0x") {
                            format!("TYPE_{}", stripped.to_uppercase())
                        } else {
                            // Convert from PascalCase to SCREAMING_SNAKE_CASE
                            let mut result = String::new();
                            let mut prev_was_lower = false;
                            for (i, ch) in variant_name.chars().enumerate() {
                                if ch.is_uppercase() && i > 0 && prev_was_lower {
                                    result.push('_');
                                }
                                result.push(ch.to_ascii_uppercase());
                                prev_was_lower = ch.is_lowercase();
                            }
                            result
                        };
                        format!("{}::{}.bits()", enum_name, const_name)
                    } else {
                        // Regular enum - cast to u32
                        format!("{}::{} as u32", enum_name, variant_name)
                    }
                } else {
                    mask_value.clone()
                }
            } else {
                mask_value.clone()
            };

            // Cast mask field to u32 if it's an enum type
            let mask_field_expr =
                generate_mask_field_expr(ctx, mask_field, &mask_field_safe, all_fields);

            // Generate: if (flags.clone() as u32 & 0x8) != 0 { read().map(Some) } else { Ok(None) }
            format!(
                "if ({} & {}) != 0 {{ {}.map(Some) }} else {{ Ok(None) }}",
                mask_field_expr, mask_value_code, base_read
            )
        } else {
            // No condition - this shouldn't happen for truly optional fields, but handle it
            base_read
        }
    } else {
        base_read
    }
}

/// Convert a condition expression from XML format to Rust code
/// Examples: "RecordCount > 0" -> "record_count > 0"
fn convert_condition_expression(expr: &str, all_fields: &[Field]) -> String {
    let expr = expr.trim();
    let mut result = String::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        if ch.is_whitespace()
            || ch == '>'
            || ch == '<'
            || ch == '='
            || ch == '!'
            || ch == '&'
            || ch == '|'
            || ch == '('
            || ch == ')'
        {
            if !current_token.is_empty() {
                // Try to find a field with this name
                if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
                    let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    result.push_str(&safe_name);
                } else if current_token.chars().all(|c| c.is_numeric()) {
                    // It's a number
                    result.push_str(&current_token);
                } else {
                    // Unknown token - keep as-is but make it snake_case
                    let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
                    result.push_str(&safe_name);
                }
                current_token.clear();
            }

            if !ch.is_whitespace() {
                result.push(ch);
            } else if !result.is_empty() && !result.ends_with(' ') {
                result.push(' ');
            }
        } else {
            current_token.push(ch);
        }
    }

    // Handle any remaining token
    if !current_token.is_empty() {
        if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
            let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
            result.push_str(&safe_name);
        } else if current_token.chars().all(|c| c.is_numeric()) {
            result.push_str(&current_token);
        } else {
            let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
            result.push_str(&safe_name);
        }
    }

    result.trim().to_string()
}

/// Convert a length expression from XML format to Rust code
/// Examples: "Count" -> "count", "RecordCount - 1" -> "(record_count - 1)"
fn convert_length_expression(expr: &str, all_fields: &[Field]) -> String {
    // Simple expression parsing - handle basic arithmetic
    let expr = expr.trim();

    // Special case: "*" means read remaining bytes - this will be handled specially in generation
    if expr == "*" {
        return "*".to_string();
    }

    // Check if it's a simple field reference
    if let Some(field) = all_fields.iter().find(|f| f.name == expr) {
        let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
        return format!("{} as usize", safe_name);
    }

    // Handle arithmetic expressions (e.g., "RecordCount - 1")
    // Split on operators and convert field names
    let mut result = String::new();
    let mut current_token = String::new();

    for ch in expr.chars() {
        if ch.is_whitespace()
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '/'
            || ch == '('
            || ch == ')'
        {
            if !current_token.is_empty() {
                // Try to find a field with this name
                if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
                    let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
                    result.push_str(&format!("{} as usize", safe_name));
                } else if current_token.chars().all(|c| c.is_numeric()) {
                    // It's a number
                    result.push_str(&current_token);
                } else {
                    // Unknown token - keep as-is but make it snake_case
                    let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
                    result.push_str(&format!("{} as usize", safe_name));
                }
                current_token.clear();
            }

            if !ch.is_whitespace() {
                result.push(' ');
                result.push(ch);
                result.push(' ');
            }
        } else {
            current_token.push(ch);
        }
    }

    // Handle any remaining token
    if !current_token.is_empty() {
        if let Some(field) = all_fields.iter().find(|f| f.name == current_token) {
            let safe_name = safe_identifier(&field.name, IdentifierType::Field).name;
            result.push_str(&format!("{} as usize", safe_name));
        } else if current_token.chars().all(|c| c.is_numeric()) {
            result.push_str(&current_token);
        } else {
            let safe_name = safe_identifier(&current_token, IdentifierType::Field).name;
            result.push_str(&format!("{} as usize", safe_name));
        }
    }

    result.trim().to_string()
}

/// Generate a Vec read with a known length
fn generate_vec_read_with_length(element_type: &str, length_code: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);

    // Special case: "*" means read all remaining bytes
    if length_code == "*" {
        // For primitive types like u8, read directly as bytes
        if rust_type == "u8" {
            // Use read_to_end which requires special handling - wrap in Ok
            return "(|| -> Result<Vec<u8>, Box<dyn std::error::Error>> {\n                let mut data = Vec::new();\n                let _ = reader.read_to_end(&mut data);\n                Ok(data)\n            })()"
                .to_string();
        } else {
            // For other types, read items until EOF - wrap in Ok
            return format!(
                "(|| -> Result<Vec<{}>, Box<dyn std::error::Error>> {{\n                let mut vec = Vec::new();\n                loop {{\n                    match read_item::<{}>(reader) {{\n                        Ok(item) => vec.push(item),\n                        Err(_) => break,\n                    }}\n                }}\n                Ok(vec)\n            }})()",
                rust_type, rust_type
            );
        }
    }

    format!("read_vec::<{}>(reader, {})", rust_type, length_code)
}

/// Generate a PackableList read (count + vector of items)
fn generate_packable_list_read(element_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let rust_type = get_rust_type(element_type);
    format!("read_packable_list::<{}>(reader)", rust_type)
}

/// Generate a PackableHashTable read (count + max_size + table of items)
fn generate_packable_hash_table_read(
    key_type: &str,
    value_type: &str,
    ctx: &ReaderContext,
) -> String {
    // Helper to generate read code for a type
    let generate_simple_read = |typ: &str| -> String {
        let rust_type = get_rust_type(typ);
        match rust_type {
            "u8" => "read_u8(reader)?".to_string(),
            "i8" => "read_i8(reader)?".to_string(),
            "u16" => "read_u16(reader)?".to_string(),
            "i16" => "read_i16(reader)?".to_string(),
            "u32" => "read_u32(reader)?".to_string(),
            "i32" => "read_i32(reader)?".to_string(),
            "u64" => "read_u64(reader)?".to_string(),
            "i64" => "read_i64(reader)?".to_string(),
            "f32" => "read_f32(reader)?".to_string(),
            "f64" => "read_f64(reader)?".to_string(),
            "bool" => "read_bool(reader)?".to_string(),
            "String" => "read_string(reader)?".to_string(),
            _ => {
                if let Some(parent_type) = ctx.enum_parent_map.get(typ) {
                    let parent_rust_type = get_rust_type(parent_type);
                    let read_fn = match parent_rust_type {
                        "u8" => "read_u8",
                        "i8" => "read_i8",
                        "u16" => "read_u16",
                        "i16" => "read_i16",
                        "u32" => "read_u32",
                        "i32" => "read_i32",
                        "u64" => "read_u64",
                        "i64" => "read_i64",
                        _ => panic!("Unsupported enum parent type: {}", parent_type),
                    };
                    // Use from_bits_retain for bitflags types, try_from for regular enums
                    if ctx.mask_enums.contains(typ) {
                        format!("{}::from_bits_retain({}(reader)?)", typ, read_fn)
                    } else {
                        format!("{}::try_from({}(reader)?)?", typ, read_fn)
                    }
                } else {
                    format!("{}::read(reader)?", typ)
                }
            }
        }
    };

    let key_read = generate_simple_read(key_type);
    let value_read = generate_simple_read(value_type);

    // Call the read_packable_hash_table_with helper function
    format!(
        "read_packable_hash_table_with(reader, |r| {{
            {}
        }}, |r| {{
            {}
        }})",
        format!("Ok({})", key_read).replace("reader", "r"),
        format!("Ok({})", value_read).replace("reader", "r")
    )
}

/// Generate a PHashTable read (packed_size + table of items)
fn generate_phash_table_read(key_type: &str, value_type: &str) -> String {
    // All types that can be read (primitives, enums, or custom structs) implement ACDataType
    let key_rust = get_rust_type(key_type);
    let value_rust = get_rust_type(value_type);
    format!("read_phash_table::<{}, {}>(reader)", key_rust, value_rust)
}

/// Generate a HashMap read with a known length
fn generate_hashmap_read_with_length(
    key_type: &str,
    value_type: &str,
    length_code: &str,
    ctx: &ReaderContext,
) -> String {
    // Helper to generate read code for a type
    let generate_simple_read = |typ: &str| -> String {
        let rust_type = get_rust_type(typ);
        match rust_type {
            "u8" => "read_u8(reader)?".to_string(),
            "i8" => "read_i8(reader)?".to_string(),
            "u16" => "read_u16(reader)?".to_string(),
            "i16" => "read_i16(reader)?".to_string(),
            "u32" => "read_u32(reader)?".to_string(),
            "i32" => "read_i32(reader)?".to_string(),
            "u64" => "read_u64(reader)?".to_string(),
            "i64" => "read_i64(reader)?".to_string(),
            "f32" => "read_f32(reader)?".to_string(),
            "f64" => "read_f64(reader)?".to_string(),
            "bool" => "read_bool(reader)?".to_string(),
            "String" => "read_string(reader)?".to_string(),
            _ => {
                if let Some(parent_type) = ctx.enum_parent_map.get(typ) {
                    let parent_rust_type = get_rust_type(parent_type);
                    let read_fn = match parent_rust_type {
                        "u8" => "read_u8",
                        "i8" => "read_i8",
                        "u16" => "read_u16",
                        "i16" => "read_i16",
                        "u32" => "read_u32",
                        "i32" => "read_i32",
                        "u64" => "read_u64",
                        "i64" => "read_i64",
                        _ => panic!("Unsupported enum parent type: {}", parent_type),
                    };
                    // Use from_bits_retain for bitflags types, try_from for regular enums
                    if ctx.mask_enums.contains(typ) {
                        format!("{}::from_bits_retain({}(reader)?)", typ, read_fn)
                    } else {
                        format!("{}::try_from({}(reader)?)?", typ, read_fn)
                    }
                } else {
                    format!("{}::read(reader)?", typ)
                }
            }
        }
    };

    let key_read = generate_simple_read(key_type);
    let value_read = generate_simple_read(value_type);

    // The block is an expression that evaluates to Result<HashMap<K, V>, Error>
    // We use ? inside the block and wrap in Ok() at the end
    format!(
        "(|| -> Result<_, Box<dyn std::error::Error>> {{\n            let length = {};\n            let mut map = std::collections::HashMap::with_capacity(length);\n            for _ in 0..length {{\n                let key = {};\n                let value = {};\n                map.insert(key, value);\n            }}\n            Ok(map)\n        }})()",
        length_code, key_read, value_read
    )
}

pub fn generate(xml: &str, filter_types: &[String]) -> GeneratedCode {
    generate_with_source(xml, filter_types, GenerateSource::Protocol)
}

/// Helper function to perform code generation with files merging from multiple sources
/// This is used by both build.rs and xtask generate to ensure consistent code generation
pub fn generate_and_merge(
    protocol_xml: &str,
    network_xml: Option<&str>,
    filter_types: &[String],
) -> GeneratedCode {
    // Generate from protocol.xml
    let mut generated_code =
        generate_with_source(protocol_xml, filter_types, GenerateSource::Protocol);

    // Generate from network.xml if provided and merge results
    if let Some(network_xml) = network_xml {
        let network_code = generate_with_source(network_xml, filter_types, GenerateSource::Network);

        // Merge files from network.xml into generated_code
        for network_file in network_code.files {
            if network_file.path.ends_with("mod.rs") {
                // For mod.rs files, merge the pub mod and pub use declarations
                if let Some(existing) = generated_code
                    .files
                    .iter_mut()
                    .find(|f| f.path == network_file.path)
                {
                    // Merge module declarations from network.xml into existing mod.rs
                    let mut merged_content = existing.content.clone();

                    // Extract pub mod and pub use lines from network file
                    for line in network_file.content.lines() {
                        let trimmed = line.trim();
                        if (trimmed.starts_with("pub mod ") || trimmed.starts_with("pub use "))
                            && !merged_content.contains(trimmed)
                        {
                            // Add this line if it's not already present
                            merged_content.push('\n');
                            merged_content.push_str(line);
                        }
                    }

                    existing.content = merged_content;
                } else {
                    // No existing mod.rs, add network's mod.rs
                    generated_code.files.push(network_file);
                }
            } else {
                // For non-mod.rs files, just add/replace them
                if let Some(pos) = generated_code
                    .files
                    .iter()
                    .position(|f| f.path == network_file.path)
                {
                    generated_code.files[pos] = network_file;
                } else {
                    generated_code.files.push(network_file);
                }
            }
        }
    }

    generated_code
}

/// Generate code from protocol XML, with source indication for packets section
pub fn generate_with_source(
    xml: &str,
    filter_types: &[String],
    source: GenerateSource,
) -> GeneratedCode {
    let ctx = GenerationContext::new(filter_types.to_vec());

    // Parse XML content using the xml_parser module
    let (
        mut enum_types,
        common_types,
        game_action_types,
        game_event_types,
        c2s_types,
        s2c_types,
        packet_types,
    ) = crate::xml_parser::parse_xml_content(xml, source);

    // Rectify dependencies between types and enums
    let mut rectified_common_types = Vec::new();
    let mut rectified_c2s_types = Vec::new();
    let mut rectified_s2c_types = Vec::new();
    rectify_dependencies(
        &common_types,
        &c2s_types,
        &s2c_types,
        &mut enum_types,
        &mut rectified_common_types,
        &mut rectified_c2s_types,
        &mut rectified_s2c_types,
    );

    // Other type collections (no rectification needed for now, just clone)
    let rectified_game_action_types = game_action_types.clone();
    let rectified_game_event_types = game_event_types.clone();
    let rectified_packet_types = packet_types.clone();

    // Generate enums
    let mut enums_out = String::new();
    enums_out.push_str("use serde::{Serialize, Deserialize};\n");
    enums_out.push_str("use num_enum::TryFromPrimitive;\n");
    enums_out.push_str("use crate::readers::ACReader;\n\n");

    for protocol_enum in &enum_types {
        if protocol_enum.is_mask {
            enums_out.push_str(&generate_bitflags(protocol_enum));
        } else {
            enums_out.push_str(&generate_enum(protocol_enum));
        }
    }

    // Generate common types - will add readers after building reader context
    let mut common_types_out = String::new();
    common_types_out.push_str("use serde::{Serialize, Deserialize};\n");
    common_types_out.push_str("use std::io::Read;\n");
    common_types_out.push_str("use crate::readers::ACReader;\n");
    common_types_out.push_str("use crate::readers::*;\n");
    common_types_out.push_str("use crate::enums::*;\n\n");

    for protocol_type in &rectified_common_types {
        if protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let rust_type = get_rust_type(type_name);
            if rust_type != type_name {
                if let Some(ref text) = protocol_type.text {
                    common_types_out.push_str(&format!("/// {text}\n"));
                }
                common_types_out.push_str(&format!(
                    "#[allow(non_camel_case_types)]\npub type {type_name} = {rust_type};\n\n"
                ));
            }
        } else {
            common_types_out.push_str(&generate_type(protocol_type));
        }
    }

    // Build a map of enum names to their parent types for reader generation
    let enum_parent_map: BTreeMap<String, String> = enum_types
        .iter()
        .map(|e| (e.name.clone(), e.parent.clone()))
        .collect();

    // Build a map of (enum_name, value) -> variant_name for switch pattern matching
    let mut enum_value_map: BTreeMap<(String, i64), String> = BTreeMap::new();
    for protocol_enum in &enum_types {
        for enum_value in &protocol_enum.values {
            let safe_variant = safe_enum_variant_name(&enum_value.name);
            enum_value_map.insert(
                (protocol_enum.name.clone(), enum_value.value),
                safe_variant.name,
            );
        }
    }

    // Build a set of mask enum names (bitflags types)
    let mask_enums: std::collections::HashSet<String> = enum_types
        .iter()
        .filter(|e| e.is_mask)
        .map(|e| e.name.clone())
        .collect();

    let reader_ctx = ReaderContext::new(enum_parent_map, enum_value_map, mask_enums);

    // Add reader implementations to common types
    for protocol_type in &rectified_common_types {
        if ctx.should_generate_reader(&protocol_type.name) {
            common_types_out.push_str(&generate_reader_impl(&reader_ctx, protocol_type));
        }
    }

    // Generate individual files for each type
    let mut files = Vec::new();

    // Add enums file
    files.push(GeneratedFile {
        path: "enums/mod.rs".to_string(),
        content: enums_out,
    });

    // Generate common types as types/mod.rs
    files.push(GeneratedFile {
        path: "types/mod.rs".to_string(),
        content: common_types_out,
    });

    // Track module names for generating mod.rs files
    let mut c2s_modules = Vec::new();
    let mut s2c_modules = Vec::new();

    // Generate individual files for C2S messages
    for protocol_type in &rectified_c2s_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            // Remove underscores first, then convert to snake_case to avoid double underscores
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            c2s_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("messages/c2s/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for S2C messages
    for protocol_type in &rectified_s2c_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            // Remove underscores first, then convert to snake_case to avoid double underscores
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            s2c_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("messages/s2c/{}.rs", module_name),
                content,
            });
        }
    }

    // Track module names for each section
    let mut game_action_modules = Vec::new();
    let mut game_event_modules = Vec::new();
    let mut packet_modules = Vec::new();
    let mut network_modules = Vec::new();

    // Generate individual files for game actions
    for protocol_type in &rectified_game_action_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            game_action_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("gameactions/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for game events
    for protocol_type in &rectified_game_event_types {
        if !protocol_type.is_primitive {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            game_event_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("gameevents/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for packet types (from protocol.xml <packets>)
    // Skip Network category types - they go to network/ folder instead
    for protocol_type in &rectified_packet_types {
        if !protocol_type.is_primitive && protocol_type.category != ProtocolCategory::Network {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            packet_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("packets/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate individual files for network types (from network.xml <packets>)
    // These are identified by having category = Network
    for protocol_type in &rectified_packet_types {
        if !protocol_type.is_primitive && protocol_type.category == ProtocolCategory::Network {
            let type_name = &protocol_type.name;
            let type_name_no_underscores = type_name.replace('_', "");
            let module_name = to_snake_case(&type_name_no_underscores);
            network_modules.push(module_name.clone());
            let content = generate_type_and_reader_file(&ctx, &reader_ctx, protocol_type);
            files.push(GeneratedFile {
                path: format!("network/{}.rs", module_name),
                content,
            });
        }
    }

    // Generate mod.rs for c2s
    let mut c2s_mod = String::new();
    for module_name in &c2s_modules {
        c2s_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    c2s_mod.push('\n');
    for module_name in &c2s_modules {
        c2s_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "messages/c2s/mod.rs".to_string(),
        content: c2s_mod,
    });

    // Generate mod.rs for s2c
    let mut s2c_mod = String::new();
    for module_name in &s2c_modules {
        s2c_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    s2c_mod.push('\n');
    for module_name in &s2c_modules {
        s2c_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "messages/s2c/mod.rs".to_string(),
        content: s2c_mod,
    });

    // Generate mod.rs for messages
    let messages_mod = "pub mod c2s;\npub mod s2c;\n";
    files.push(GeneratedFile {
        path: "messages/mod.rs".to_string(),
        content: messages_mod.to_string(),
    });

    // Generate mod.rs for gameactions
    let mut gameactions_mod = String::new();
    for module_name in &game_action_modules {
        gameactions_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    gameactions_mod.push('\n');
    for module_name in &game_action_modules {
        gameactions_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "gameactions/mod.rs".to_string(),
        content: gameactions_mod,
    });

    // Generate mod.rs for gameevents
    let mut gameevents_mod = String::new();
    for module_name in &game_event_modules {
        gameevents_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    gameevents_mod.push('\n');
    for module_name in &game_event_modules {
        gameevents_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "gameevents/mod.rs".to_string(),
        content: gameevents_mod,
    });

    // Generate mod.rs for packets (from protocol.xml <packets>)
    let mut packets_mod = String::new();
    for module_name in &packet_modules {
        packets_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    packets_mod.push('\n');
    for module_name in &packet_modules {
        packets_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "packets/mod.rs".to_string(),
        content: packets_mod,
    });

    // Generate mod.rs for network (from network.xml <packets>)
    let mut network_mod = String::new();
    for module_name in &network_modules {
        network_mod.push_str(&format!("pub mod {};\n", module_name));
    }
    network_mod.push('\n');
    for module_name in &network_modules {
        network_mod.push_str(&format!("pub use {}::*;\n", module_name));
    }
    files.push(GeneratedFile {
        path: "network/mod.rs".to_string(),
        content: network_mod,
    });

    // Generate root mod.rs for generated
    let generated_mod = "pub mod enums;\npub mod types;\npub mod messages;\npub mod gameactions;\npub mod gameevents;\npub mod packets;\npub mod network;\n";
    files.push(GeneratedFile {
        path: "mod.rs".to_string(),
        content: generated_mod.to_string(),
    });

    GeneratedCode { files }
}
