/// Convert PascalCase/camelCase to snake_case
pub fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;

    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase if:
            // - Not the first character
            // - Previous character was lowercase or digit
            if i > 0 && (prev_is_lower || name.chars().nth(i - 1).is_some_and(|c| c.is_numeric())) {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_is_lower = false;
        } else {
            result.push(ch);
            prev_is_lower = ch.is_lowercase();
        }
    }

    result
}

const RUST_RESERVED_WORDS: &[&str] = &["Self", "type"];

/// Check if a field name is a Rust reserved word
pub fn is_reserved_word(name: &str) -> bool {
    RUST_RESERVED_WORDS.contains(&name)
}

/// Convert a field name to a safe Rust identifier in snake_case
pub fn safe_field_name(name: &str) -> (String, bool) {
    let snake_case = to_snake_case(name);

    if is_reserved_word(&snake_case) {
        let safe_name = format!("{snake_case}_");
        (safe_name, true)
    } else {
        let needs_rename = name != snake_case;
        (snake_case, needs_rename)
    }
}
