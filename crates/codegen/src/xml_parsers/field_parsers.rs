use log::debug;
use quick_xml::events::BytesStart;

use crate::{
    field_gen::FieldContext,
    types::{Field, FieldSet, IfBranch},
};

/// Create a field from a tag in the XML
pub fn create_field_from_tag(e: &quick_xml::events::BytesStart, ctx: &FieldContext) -> Option<Field> {
    let mut field_type = None;
    let mut field_name = None;
    let mut generic_key = None;
    let mut generic_value = None;
    let mut generic_type = None;
    let mut param = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"type" => field_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"name" => field_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericKey" => generic_key = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericValue" => generic_value = Some(attr.unescape_value().unwrap().into_owned()),
            b"genericType" => generic_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"param" => param = Some(attr.unescape_value().unwrap().into_owned()),
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
            param,
        })
    } else {
        None
    }
}

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

/// Process a field tag in the XML
pub fn process_field_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    if let Some(new_field) = create_field_from_tag(e, ctx) {
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

        add_field_to_set(new_field, current_field_set, ctx);
    }
}

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

/// Process a table tag in the XML
pub fn process_table_tag(
    e: &quick_xml::events::BytesStart,
    current_field_set: &mut Option<FieldSet>,
    ctx: &mut FieldContext,
) {
    let mut table_name = None;
    let mut key_type = None;
    let mut value_type = None;
    let mut length_expr = None;

    for attr in e.attributes().flatten() {
        match attr.key.as_ref() {
            b"name" => table_name = Some(attr.unescape_value().unwrap().into_owned()),
            b"key" => key_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"value" => value_type = Some(attr.unescape_value().unwrap().into_owned()),
            b"length" => length_expr = Some(attr.unescape_value().unwrap().into_owned()),
            _ => {}
        }
    }

    debug!(
        "Processing table {table_name:?} with key={key_type:?}, value={value_type:?}, length={length_expr:?}"
    );

    if let (Some(tname), Some(ktype), Some(vtype)) = (table_name, key_type, value_type) {
        // Create a field with HashMap<K, V> type
        let map_type = format!("std::collections::HashMap<{ktype}, {vtype}>");
        let is_optional = ctx.in_if_true || ctx.in_if_false || ctx.in_maskmap;

        let if_branch = if ctx.in_if_true {
            Some(IfBranch::True)
        } else if ctx.in_if_false {
            Some(IfBranch::False)
        } else {
            None
        };

        let new_field = Field {
            name: tname,
            field_type: map_type,
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
            debug!("Added table to if_true_fields");
            return;
        } else if ctx.in_if_false {
            ctx.if_false_fields.push(new_field);
            debug!("Added table to if_false_fields");
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
                        debug!("Added table to variant case {case_val}");
                    }
                }
            } else {
                field_set.common_fields.push(new_field);
                debug!("Added table to common_fields");
            }
        }
    }
}