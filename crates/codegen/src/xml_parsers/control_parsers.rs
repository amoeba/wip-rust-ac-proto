use log::debug;
use quick_xml::events::BytesStart;

use crate::{
    types::{Field, FieldSet, Subfield, ProtocolCategory},
    util::parse_enum_value,
};

/// Process a switch tag in the XML
pub fn process_switch_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
) -> bool {
    let mut name: Option<String> = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"name" {
            name = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    if let (Some(switch_name), Some(field_set)) = (name, current_field_set) {
        field_set.switch_field = Some(switch_name);
        field_set.variant_fields = Some(std::collections::BTreeMap::new());
        debug!(
            "Entered switch, switch_field = {:?}",
            field_set.switch_field
        );
        true
    } else {
        false
    }
}

/// Process a case tag in the XML
pub fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<Vec<i64>> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    // Parse multi-value cases (e.g., "0x01 | 0x08 | 0x0A") into individual parsed i64 values
    let values = value.map(|v| {
        v.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                parse_enum_value(s)
                    .map_err(|e| {
                        eprintln!("Warning: {}", e);
                        e
                    })
                    .ok()
            })
            .collect::<Vec<i64>>()
    });

    debug!("current_case_value is now {values:?}");
    values
}

/// Process a subfield tag in the XML
pub fn process_subfield_tag(e: &quick_xml::events::BytesStart) -> Option<Subfield> {
    let mut name = None;
    let mut field_type = None;
    let mut value_expression = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value_expression = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    if let (Some(name), Some(field_type), Some(value_expression)) =
        (name, field_type, value_expression)
    {
        Some(Subfield {
            name,
            field_type,
            value_expression,
        })
    } else {
        None
    }
}

/// Process an align tag in the XML
pub fn process_align_tag(e: &quick_xml::events::BytesStart) -> Option<Field> {
    let mut align_type = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"type" {
            align_type = Some(attr.unescape_value().unwrap().into_owned());
        }
    }

    if let Some(align_to) = align_type {
        // Generate a synthetic field name for alignment padding
        let padding_field_name = format!("align_{}", align_to.to_lowercase());

        // Map alignment type to read call
        // We generate code that reads the padding needed to align to the specified boundary
        Some(Field {
            name: padding_field_name,
            field_type: format!("__align__{}", align_to),
            is_optional: false,
            length_expression: None,
            optional_condition: None,
            mask_field: None,
            mask_value: None,
            if_branch: None,
            if_false_branch_type: None,
            subfields: Vec::new(),
            nested_field_set: None,
            param: None,
        })
    } else {
        None
    }
}