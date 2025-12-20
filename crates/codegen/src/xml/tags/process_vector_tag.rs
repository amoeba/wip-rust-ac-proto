use log::debug;

use crate::{
    field_gen::FieldContext,
    types::{Field, FieldSet, IfBranch},
};

/// Process a vector tag in the XML
pub fn process_vector_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    let mut vector_type = None;
    let mut vector_name = None;
    let mut length_expr = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => vector_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => vector_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"length" => length_expr = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!("Processing vector {vector_name:?} of type {vector_type:?} with length {length_expr:?}");

    if let (Some(vname), Some(vtype)) = (vector_name, vector_type) {
        // Create a field with Vec<T> type
        let vec_type = format!("Vec<{vtype}>");
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        let new_field = Field {
            name: vname,
            field_type: vec_type,
            is_optional,
            length_expression: length_expr,
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
            param: None,
        };

        // If we're in an <if> block, collect fields separately
        if ctx.in_if_true {
            ctx.if_true_fields.push(new_field);
            debug!("Added vector to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added vector to if_false_fields");
            return;
        }

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
                            .push(new_field.clone());
                        debug!("Added vector to variant case {case_val}");
                    }
                }
            } else {
                field_set.common_fields.push(new_field);
            }
        }
    }
}
