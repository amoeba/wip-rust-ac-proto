use quick_xml::events::BytesStart;

use crate::{
    types::{ProtocolEnum, EnumValue},
    util::parse_enum_value,
};

/// Process an enum value tag in the XML
pub fn process_enum_value_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut value = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    if let (Some(name), Some(value_str), Some(current_enum)) = (name, value, current_enum) {
        // Handle multiple values in a single attribute (e.g., "0x0C | 0x0D")
        let values: Vec<&str> = value_str.split('|').collect();
        for val in values {
            let trimmed_val = val.trim();
            if !trimmed_val.is_empty() {
                match parse_enum_value(trimmed_val) {
                    Ok(parsed_value) => {
                        let enum_value = EnumValue {
                            name: name.clone(),
                            value: parsed_value,
                        };
                        current_enum.values.push(enum_value);
                    }
                    Err(e) => {
                        eprintln!("Warning: {}", e);
                    }
                }
            }
        }
    }
}