use std::collections::BTreeMap;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, FieldSet, IfBranch},
};

/// A group of consecutive fields with the same condition
#[derive(Debug)]
pub struct FieldGroup<'a> {
    pub condition: ConditionKey,
    pub fields: Vec<&'a Field>,
}

/// Key for grouping fields by their condition
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConditionKey {
    /// No condition - always read
    None,
    /// <if test="condition">
    IfTest(String),
    /// <maskmap> with specific field and mask value
    Mask { field: String, value: String },
}

impl ConditionKey {
    pub fn from_field(field: &Field) -> Self {
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

/// Groups consecutive fields by their condition to reduce repetitive conditional checks
pub fn group_consecutive_fields_by_condition(fields: &[Field]) -> Vec<FieldGroup<'_>> {
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

/// Generate reads for a group of fields with the same condition
pub fn generate_field_group_reads(
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
                    // Generate a read call for the false branch (this should be handled properly by the specific function)
                    let read_call = generate_read_base_logic(ctx, field, all_fields);
                    out.push_str(&format!(
                        "            {} = Some({}?);\n",
                        field_name, read_call
                    ));
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
            // Initialize all optional fields to None first
            for field in fields {
                let field_name = safe_identifier(&field.name, IdentifierType::Field).name;
                out.push_str(&format!("        let mut {} = None;\n", field_name));
            }

            // Generate the if block
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

use super::context::ReaderContext;

/// Generate the appropriate read call for a given field
///
/// # Arguments
/// * `ctx` - Reader context with enum information
/// * `field` - The field to generate a read call for
/// * `all_fields` - All fields available in the current scope (used to resolve length expressions)
pub fn generate_read_call(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
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

/// Helper function that generates base read call without optional field handling
fn generate_read_base_logic(ctx: &ReaderContext, field: &Field, all_fields: &[Field]) -> String {
    let field_type = &field.field_type;

    // Handle alignment padding fields first
    if let Some(align_type) = field_type.strip_prefix("__align__") {
        let padding_code = match align_type {
            "DWORD" => "align_dword(reader)",
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

/// Convert a condition expression from XML format to Rust code
/// Examples: "RecordCount > 0" -> "record_count > 0"
pub fn convert_condition_expression(expr: &str, all_fields: &[Field]) -> String {
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
pub fn convert_length_expression(expr: &str, all_fields: &[Field]) -> String {
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
            Ok({})
        }}, |r| {{
            Ok({})
        }})",
        key_read.replace("reader", "r"),
        value_read.replace("reader", "r")
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
