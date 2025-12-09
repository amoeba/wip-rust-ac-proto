use std::collections::BTreeMap;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, FieldSet, IfBranch},
};

// Re-export functions from read_generation submodules
pub use super::read_generation::expression_readers::convert_length_expression;
pub use super::read_generation::primitive_readers::{
    convert_condition_expression, generate_read_call,
};
pub use super::read_generation::{
    ConditionKey, FieldGroup, generate_field_group_reads, group_consecutive_fields_by_condition,
};
