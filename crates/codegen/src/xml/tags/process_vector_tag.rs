use log::debug;

use crate::{
    field_gen::{FieldContext, build_field_from_context},
    types::FieldSet,
};

/// Process a vector tag in the XML
pub fn process_vector_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    crate::extract_xml_attrs!(e, {
        b"type" => vector_type,
        b"name" => vector_name,
        b"length" => length_expr
    });

    debug!("Processing vector {vector_name:?} of type {vector_type:?} with length {length_expr:?}");

    if let (Some(vname), Some(vtype)) = (vector_name, vector_type) {
        // Create a field with Vec<T> type
        let vec_type = format!("Vec<{vtype}>");
        let new_field = build_field_from_context(vname, vec_type, length_expr, None, ctx);

        crate::xml::utils::route_field(new_field, current_field_set, ctx);
    }
}
