use crate::{
    identifiers::{IdentifierType, safe_identifier},
    types::Field,
};

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

    // Special case: "*" means read remaining bytes - this will be handled specially
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
