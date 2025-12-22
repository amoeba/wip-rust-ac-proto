use crate::{field_gen::FieldContext, types::FieldSet};

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
        crate::xml::utils::route_field(new_field, current_field_set, ctx);
    }
}
