use log::debug;
use quick_xml::events::BytesStart;

use crate::{
    types::{EnumValue, ProtocolCategory, ProtocolEnum},
    util::parse_enum_value,
};

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
