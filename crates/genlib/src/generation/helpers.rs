use std::collections::BTreeMap;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, IfBranch},
};

// Re-export functions from read_generation module
pub use super::read_generation::{
    ConditionKey, FieldGroup, convert_condition_expression, convert_length_expression,
    generate_field_group_reads, generate_read_call, group_consecutive_fields_by_condition,
};
