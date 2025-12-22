use log::debug;

use crate::{
    field_gen::{FieldContext, build_field_from_context},
    types::FieldSet,
};

/// Process a table tag in the XML
pub fn process_table_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    crate::extract_xml_attrs!(e, {
        b"name" => table_name,
        b"key" => key_type,
        b"value" => value_type,
        b"length" => length_expr
    });

    debug!(
        "Processing table {table_name:?} with key={key_type:?}, value={value_type:?}, length={length_expr:?}"
    );

    if let (Some(tname), Some(ktype), Some(vtype)) = (table_name, key_type, value_type) {
        // Create a field with HashMap<K, V> type
        let map_type = format!("std::collections::HashMap<{ktype}, {vtype}>");
        let new_field = build_field_from_context(tname, map_type, length_expr, None, ctx);

        crate::xml::utils::route_field(new_field, current_field_set, ctx);
    }
}
