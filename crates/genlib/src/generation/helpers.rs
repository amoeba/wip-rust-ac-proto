use std::collections::BTreeMap;

use crate::{
    field_gen::get_allow_unused_directive,
    identifiers::{IdentifierType, safe_identifier, to_snake_case},
    type_utils::get_rust_type,
    types::{Field, FieldSet, IfBranch},
};

use super::types::GeneratedFile;

// Re-export functions from read_generation submodules
pub use super::read_generation::expression_readers::convert_length_expression;
pub use super::read_generation::primitive_readers::{
    convert_condition_expression, generate_read_call,
};
pub use super::read_generation::{
    ConditionKey, FieldGroup, generate_field_group_reads, group_consecutive_fields_by_condition,
};

/// Generate a module file (mod.rs) that declares and re-exports a list of modules
pub fn generate_module_file(modules: &[String], path: &str) -> GeneratedFile {
    let mut content = String::new();

    // Add pub mod declarations
    for module_name in modules {
        content.push_str(&format!("pub mod {};\n", module_name));
    }

    content.push('\n');

    // Add pub use re-exports
    for module_name in modules {
        content.push_str(&format!("pub use {}::*;\n", module_name));
    }

    GeneratedFile {
        path: path.to_string(),
        content,
    }
}
