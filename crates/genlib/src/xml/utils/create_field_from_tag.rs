use log::debug;

use crate::{
    field_gen::FieldContext,
    types::{Field, IfBranch},
};

/// Create a field from a tag in the XML
pub fn create_field_from_tag(
    e: &quick_xml::events::BytesStart,
    ctx: &FieldContext,
) -> Option<Field> {
    let mut field_type = None;
    let mut field_name = None;
    let mut generic_key = None;
    let mut generic_value = None;
    let mut generic_type = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => field_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericKey" => generic_key = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericValue" => generic_value = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericType" => generic_type = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing field {field_name:?}");

    if let (Some(fname), Some(mut ftype)) = (field_name, field_type) {
        // Handle generic types
        if let (Some(key), Some(value)) = (generic_key, generic_value) {
            // PackableHashTable<K, V>
            ftype = format!("{ftype}<{key}, {value}>");
        } else if let Some(gtype) = generic_type {
            // PackableList<T>
            ftype = format!("{ftype}<{gtype}>");
        }
        // Fields inside <if>/<true>/<false> or <maskmap> blocks are conditional
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        // Track which branch this field belongs to (used for if/true/false code generation)
        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        Some(Field {
            name: fname,
            field_type: ftype,
            is_optional,
            length_expression: None, // Regular fields don't have length expressions
            optional_condition: if is_optional {
                ctx.current_if_condition.clone()
            } else {
                None
            },
            mask_field: if ctx.in_maskmap {
                ctx.current_maskmap_field.clone()
            } else {
                None
            },
            mask_value: if ctx.in_maskmap {
                ctx.current_mask_value.clone()
            } else {
                None
            },
            if_branch,
            if_false_branch_type: None,
            subfields: Vec::new(),
            nested_field_set: None,
        })
    } else {
        None
    }
}
