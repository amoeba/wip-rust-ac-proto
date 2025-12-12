use std::collections::BTreeMap;

use log::debug;
use quick_xml::events::BytesStart;

use crate::{
    types::{ProtocolType, ProtocolEnum, EnumValue, ProtocolCategory, FieldSet},
    util::parse_enum_value,
};

/// Process a type tag in the XML
pub fn process_type_tag(
    e: &quick_xml::events::BytesStart,
    is_self_closing: bool,
    types: &mut Vec<ProtocolType>,
    current_type: &mut Option<ProtocolType>,
    current_field_set: &mut Option<FieldSet>,
) {
    let mut name = None;
    let mut text = None;
    let mut is_primitive = false;
    let mut parent = None;
    let mut templated = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"primitive" => {
                is_primitive = attr.unescape_value().unwrap() == "true";
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"templated" => {
                templated = Some(attr.unescape_value().unwrap().into_owned());
            }
            _ => {}
        }
    }

    if let Some(name) = name {
        let new_type = ProtocolType {
            name,
            text,
            fields: None,
            is_primitive,
            parent,
            templated,
            hash_bounds: Vec::new(),
            extra_derives: Vec::new(),
            category: ProtocolCategory::Types, // Will be set properly during parsing
        };

        // For self-closing tags, push immediately
        // For opening tags, set as current_type for further processing
        if is_self_closing {
            types.push(new_type);
        } else {
            *current_type = Some(new_type);
            *current_field_set = Some(FieldSet {
                switch_field: None,
                common_fields: Vec::new(),
                variant_fields: None,
                nested_switches: None,
            });
        }

        debug!("Processed type, is_self_closing={is_self_closing}");
    }
}

/// Process an enum tag start in the XML
pub fn process_enum_start_tag(
    e: &quick_xml::events::BytesStart,
    current_enum: &mut Option<ProtocolEnum>,
) {
    let mut name = None;
    let mut text = None;
    let mut parent = None;
    let mut is_mask = false;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"text" => {
                text = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"parent" => {
                parent = Some(attr.unescape_value().unwrap().into_owned());
            }
            b"mask" => {
                let mask_val = attr.unescape_value().unwrap();
                is_mask = mask_val == "true";
            }
            _ => {}
        }
    }

    if let Some(name) = name
        && let Some(parent) = parent
    {
        let new_enum = ProtocolEnum {
            name,
            text,
            parent,
            values: Vec::new(),
            extra_derives: Vec::new(),
            is_mask,
            category: ProtocolCategory::Enums, // Will be set properly during parsing
        };
        *current_enum = Some(new_enum);
    }
}

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