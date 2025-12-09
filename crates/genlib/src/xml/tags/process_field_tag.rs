use log::debug;

use crate::{
    field_gen::FieldContext,
    types::{FieldSet},
};

#[allow(unused_imports)]
use crate::types::Field;

/// Process a field tag in the XML
pub fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    if let Some(new_field) = crate::xml::utils::create_field_from_tag::create_field_from_tag(e, ctx)
    {
        // If we're in an <if> block, collect fields separately
        if ctx.in_if_true {
            ctx.if_true_fields.push(new_field);
            debug!("Added field to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added field to if_false_fields");
            return;
        }

        crate::xml::utils::add_field_to_set::add_field_to_set(new_field, current_field_set, ctx);
    }
}
