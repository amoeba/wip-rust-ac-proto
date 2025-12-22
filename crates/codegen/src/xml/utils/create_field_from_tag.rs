use log::debug;

use crate::{field_gen::{FieldContext, build_field_from_context}, types::Field};

/// Create a field from a tag in the XML
pub fn create_field_from_tag(
    e: &quick_xml::events::BytesStart,
    ctx: &FieldContext,
) -> Option<Field> {
    crate::extract_xml_attrs!(e, {
        b"type" => field_type,
        b"name" => field_name,
        b"genericKey" => generic_key,
        b"genericValue" => generic_value,
        b"genericType" => generic_type,
        b"param" => param
    });

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

        Some(build_field_from_context(
            fname,
            ftype,
            None, // Regular fields don't have length expressions
            param,
            ctx,
        ))
    } else {
        None
    }
}
