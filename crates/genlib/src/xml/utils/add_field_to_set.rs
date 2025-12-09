use log::debug;
use quick_xml::events::BytesStart;

use crate::{
    field_gen::FieldContext,
    types::{Field, FieldSet},
};

/// Add a field to a field set
pub fn add_field_to_set(
    field: Field,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    // If we're in an <if> block, this shouldn't be called - fields are handled separately
    // This is just for normal fields

    // If we're in a nested switch, route fields to the nested FieldSet instead
    if ctx.switch_nesting_level > 1 {
        if ctx.collecting_nested_trailing_fields {
            // Collecting fields after the nested switch ends
            ctx.nested_switch_trailing_fields.push(field);
            debug!("Added field to nested switch trailing fields (Empty event)");
        } else if let Some(ref mut nested) = ctx.nested_field_set {
            if let Some(ref case_vals) = ctx.current_case_values {
                // We're in a nested case
                if let Some(variant_fields) = &mut nested.variant_fields {
                    for case_val in case_vals {
                        variant_fields
                            .entry(*case_val)
                            .or_insert_with(Vec::new)
                            .push(field.clone());
                        debug!("Added field to nested switch case {case_val} (Empty event)");
                    }
                }
            } else {
                // Before the nested switch
                nested.common_fields.push(field.clone());
                ctx.nested_switch_common_fields.push(field);
                debug!("Added field to nested switch common fields (Empty event)");
            }
        }
        return;
    }

    // Normal field processing
    if let Some(field_set) = current_field_set {
        if ctx.in_switch {
            if let (Some(case_vals), Some(variant_fields)) =
                (&ctx.current_case_values, &mut field_set.variant_fields)
            {
                // Add the same field to all values in this case
                for case_val in case_vals {
                    variant_fields
                        .entry(*case_val)
                        .or_insert_with(Vec::new)
                        .push(field.clone());
                    debug!("Added field to variant case {case_val}");
                }
            }
        } else {
            field_set.common_fields.push(field);
            debug!("Added field to common_fields");
        }
    }
}
