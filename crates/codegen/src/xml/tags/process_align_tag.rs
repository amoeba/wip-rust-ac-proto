use crate::types::Field;

/// Process an align tag in the XML
pub fn process_align_tag(e: &quick_xml::events::BytesStart) -> Option<Field> {
    crate::extract_xml_attrs!(e, {
        b"type" => align_type
    });

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
