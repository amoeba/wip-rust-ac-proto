use crate::types::Subfield;

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
