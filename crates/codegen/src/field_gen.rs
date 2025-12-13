use crate::{
    identifiers::{IdentifierType, safe_identifier},
    type_utils::{convert_xml_type_to_rust, get_larger_type},
    types::{Field, IfBranch},
};

/// Static configuration for fields that should be allowed to be dead code
/// Format: (type_name, variable_name)
pub const ALLOW_DEAD_CODE_VARIABLES: &[(&str, &str)] = &[
    ("ItemProfile", "amount"),
    ("PackedMotionCommand", "server_action_sequence"),
    ("PackedMotionCommand", "autonomous"),
    ("DataId", "dat_file_type"),
    ("DDDRevision", "dat_file_type"),
    ("CommunicationTurbineChat", "blob_dispatch_type"),
];

/// Check if a variable should be allowed to be dead code (unused)
pub fn should_allow_dead_code(type_name: &str, variable_name: &str) -> bool {
    ALLOW_DEAD_CODE_VARIABLES
        .iter()
        .any(|(t, v)| *t == type_name && *v == variable_name)
}

/// Get the allow directive if the variable should allow dead code
pub fn get_allow_unused_directive(type_name: &str, variable_name: &str) -> &'static str {
    if should_allow_dead_code(type_name, variable_name) {
        "        #[allow(unused_variables)]\n"
    } else {
        ""
    }
}

/// Merge fields from <if test="..."> <true> and <false> branches
///
/// This preserves branch information for code generation. The generated code will emit
/// if-else blocks that read different fields in each branch, rather than making all
/// fields optional.
///
/// Logic:
/// - Fields only in one branch: keep with IfBranch set (True or False)
/// - Fields in both branches with same type: mark as IfBranch::Both, make optional
/// - Fields in both branches with different types: use larger type, mark as IfBranch::Both,
///   make optional, and store the false branch type for casting during code generation
pub fn merge_if_fields(mut true_fields: Vec<Field>, mut false_fields: Vec<Field>) -> Vec<Field> {
    use std::collections::HashMap;

    let mut true_map: HashMap<String, Field> =
        true_fields.drain(..).map(|f| (f.name.clone(), f)).collect();
    let mut false_map: HashMap<String, Field> = false_fields
        .drain(..)
        .map(|f| (f.name.clone(), f))
        .collect();

    let mut result = Vec::new();

    // Process all unique field names
    let all_names: std::collections::HashSet<_> =
        true_map.keys().chain(false_map.keys()).cloned().collect();

    for name in all_names {
        match (true_map.remove(&name), false_map.remove(&name)) {
            (Some(mut true_field), Some(false_field)) => {
                // Field exists in both branches
                if true_field.field_type != false_field.field_type {
                    // Different types - use the larger one and store the false branch type
                    let larger_type =
                        get_larger_type(&true_field.field_type, &false_field.field_type);
                    true_field.if_false_branch_type = Some(false_field.field_type.clone());
                    true_field.field_type = larger_type;
                }
                // Mark as existing in both branches - but don't make optional since it exists in both
                // The conditional reading will handle which value to use, but the field itself is not optional
                true_field.if_branch = Some(IfBranch::Both);
                // Don't set is_optional = true for IfBranch::Both fields, since they are always present
                result.push(true_field);
            }
            (Some(field), None) => {
                // Field only in true branch - keep IfBranch::True
                result.push(field);
            }
            (None, Some(field)) => {
                // Field only in false branch - keep IfBranch::False
                result.push(field);
            }
            (None, None) => unreachable!(),
        }
    }

    // Sort by name for consistent output
    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}

pub fn generate_field_line(field: &Field, is_enum_variant: bool) -> String {
    let original_name = &field.name;
    let safe_id = safe_identifier(original_name, IdentifierType::Field);
    let mut rust_type = convert_xml_type_to_rust(&field.field_type);

    // Wrap in Option if the field is optional
    if field.is_optional {
        rust_type = format!("Option<{rust_type}>");
    }

    // Enum variant fields can't have pub visibility
    let visibility = if is_enum_variant { "" } else { "pub " };

    // Build serde attributes
    let mut serde_attrs = Vec::new();
    if safe_id.needs_rename {
        serde_attrs.push(format!("rename = \"{original_name}\""));
    }
    if field.is_optional {
        serde_attrs.push("skip_serializing_if = \"Option::is_none\"".to_string());
    }

    if !serde_attrs.is_empty() {
        format!(
            "    #[serde({})]\n    {}{}: {rust_type}",
            serde_attrs.join(", "),
            visibility,
            safe_id.name
        )
    } else {
        format!("    {}{}: {rust_type}", visibility, safe_id.name)
    }
}

/// Default derives for all generated types
pub const DEFAULT_DERIVES: &[&str] = &["Clone", "Debug", "PartialEq", "Serialize", "Deserialize"];

/// Default derives for all generated enums (includes TryFromPrimitive for safe conversion)
pub const DEFAULT_ENUM_DERIVES: &[&str] = &[
    "Clone",
    "Debug",
    "PartialEq",
    "Serialize",
    "Deserialize",
    "TryFromPrimitive",
];

/// Build derive macro string from default derives plus extra derives
#[allow(dead_code)]
pub fn build_derive_string(extra_derives: &[String]) -> String {
    let mut all_derives: Vec<String> = DEFAULT_DERIVES.iter().map(|s| s.to_string()).collect();
    all_derives.extend(extra_derives.iter().cloned());
    format!("#[derive({})]", all_derives.join(", "))
}

/// Context for field processing within conditional blocks
#[derive(Debug)]
pub struct FieldContext {
    pub in_switch: bool,
    pub current_case_values: Option<Vec<i64>>, // Parsed numeric case values
    pub in_if_true: bool,
    pub in_if_false: bool,
    pub in_maskmap: bool,
    pub if_true_fields: Vec<Field>,
    pub if_false_fields: Vec<Field>,
    /// The test condition from the current <if> block (e.g., "RecordCount > 0")
    pub current_if_condition: Option<String>,
    /// The field name being used for maskmap (e.g., "Flags")
    pub current_maskmap_field: Option<String>,
    /// The current mask value (e.g., "0x8")
    pub current_mask_value: Option<String>,
    /// Track when we're inside a field tag (to collect subfields)
    pub in_field: bool,
    /// The current field being built (accumulates subfields)
    pub current_field: Option<Field>,
    /// Track nesting level for switches (0 = outer, 1+ = nested)
    pub switch_nesting_level: usize,
    /// For nested switches: track the outer case value we're in
    pub current_outer_case_value: Option<i64>,
    /// For nested switches: accumulated fields before the nested switch
    pub nested_switch_common_fields: Vec<Field>,
    /// For nested switches: accumulated fields after the nested switch (trailing fields)
    pub nested_switch_trailing_fields: Vec<Field>,
    /// For nested switches: the FieldSet being built for a nested switch
    pub nested_field_set: Option<crate::types::FieldSet>,
    /// Whether we're currently collecting trailing fields for a nested switch
    pub collecting_nested_trailing_fields: bool,
}
