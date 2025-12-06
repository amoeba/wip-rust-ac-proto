use core::panic;
use std::collections::HashMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    types::{Field, FieldSet, ProtocolType},
    util::safe_field_name,
};

mod types;
mod util;

fn generate_field_line(field: &Field) -> String {
    let original_name = &field.name;
    let (safe_name, needs_rename) = safe_field_name(original_name);

    if needs_rename {
        format!("        #[serde(rename = \"{original_name}\")]\n        {safe_name}: String")
    } else {
        format!("        {safe_name}: String")
    }
}

fn generate_type(protocol_type: &ProtocolType) -> String {
    let type_name = &protocol_type.name;
    println!("generate_type: name = {type_name}");

    let mut out = String::new();

    if let Some(text_str) = &protocol_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    let Some(field_set) = &protocol_type.fields else {
        // No fields, generate empty struct
        out.push_str(&format!(
            "#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct {type_name} {{}}\n\n"
        ));
        return out;
    };

    // Check if this is a variant type (has switch) or simple type
    if let Some(ref variant_fields) = field_set.variant_fields {
        // Generate enum
        let switch_field = field_set.switch_field.as_ref().unwrap();
        out.push_str(&format!(
            "#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = \"{switch_field}\")]
pub enum {type_name} {{\n"
        ));

        for (case_value, case_fields) in variant_fields {
            // Generate variant name from case value (e.g., "0x4" -> "Type4")
            let variant_name = if case_value.starts_with("0x") {
                format!("Type{}", &case_value[2..])
            } else {
                format!("Type{}", case_value)
            };

            out.push_str(&format!("    #[serde(rename = \"{case_value}\")]\n"));
            out.push_str(&format!("    {variant_name} {{\n"));

            // Add common fields first (excluding the switch field itself, as serde uses it as the tag)
            for field in &field_set.common_fields {
                if field.name != *switch_field {
                    out.push_str(&generate_field_line(field));
                    out.push_str(",\n");
                }
            }

            // Add variant-specific fields
            for field in case_fields {
                out.push_str(&generate_field_line(field));
                out.push_str(",\n");
            }

            out.push_str("    },\n");
        }

        out.push_str("}\n\n");
    } else {
        // Generate struct
        let mut field_out: Vec<String> = Vec::new();

        for field in &field_set.common_fields {
            field_out.push(generate_field_line(field));
        }

        let fields_out: String = field_out.join(",\n");

        out.push_str(&format!(
            "#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct {type_name} {{
{fields_out}
}}\n\n"
        ));
    }

    out
}

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut out = String::new();

    let mut types: Vec<ProtocolType> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    let mut current_field_set: Option<FieldSet> = None;
    let mut current_case_value: Option<String> = None;
    let mut in_switch = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().0).expect("Failed to to decode tag name");

                if tag_name == "type" {
                    let mut name = None;
                    let mut text = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"name" => name = Some(attr.unescape_value().unwrap().into_owned()),
                            b"text" => {
                                text = Some(attr.unescape_value().unwrap().into_owned());
                            }
                            _ => {}
                        }
                    }

                    if let Some(name) = name {
                        // Skip type name based on active filters
                        if let Some(ref filters) = filter_types {
                            if !filters.contains(&name) {
                                debug!("Skipping type {name} because it's not in filter list.");
                                continue;
                            }
                        }
                        current_type = Some(ProtocolType {
                            name,
                            text,
                            fields: None,
                        });
                        // Initialize a new FieldSet for this type
                        current_field_set = Some(FieldSet {
                            switch_field: None,
                            common_fields: Vec::new(),
                            variant_fields: None,
                        });

                        debug!("current_type is now {current_type:?}");
                    }
                } else if tag_name == "field" {
                    let mut field_type = None;
                    let mut field_name = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"type" => {
                                field_type = Some(attr.unescape_value().unwrap().into_owned())
                            }
                            b"name" => {
                                field_name = Some(attr.unescape_value().unwrap().into_owned())
                            }
                            _ => {}
                        }
                    }

                    debug!("Processing field {field_name:?}");

                    if let (Some(fname), Some(ftype), Some(field_set)) =
                        (field_name, field_type, &mut current_field_set)
                    {
                        let new_field = Field {
                            name: fname,
                            field_type: ftype,
                        };

                        if in_switch {
                            // We're inside a switch, add to variant fields
                            if let (Some(case_val), Some(variant_fields)) =
                                (&current_case_value, &mut field_set.variant_fields)
                            {
                                variant_fields
                                    .entry(case_val.clone())
                                    .or_insert_with(Vec::new)
                                    .push(new_field);
                                debug!("Added field to variant case {case_val}");
                            }
                        } else {
                            // Not in switch, add to common fields
                            field_set.common_fields.push(new_field);
                            debug!("Added field to common_fields");
                        }
                    }
                } else if tag_name == "switch" {
                    let mut name: Option<String> = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            name = Some(attr.unescape_value().unwrap().into_owned())
                        }
                    }

                    if let (Some(switch_name), Some(field_set)) = (name, &mut current_field_set) {
                        field_set.switch_field = Some(switch_name);
                        field_set.variant_fields = Some(HashMap::new());
                        in_switch = true;
                        debug!("Entered switch, switch_field = {:?}", field_set.switch_field);
                    }
                } else if tag_name == "case" {
                    let mut value = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            value = Some(attr.unescape_value().unwrap().into_owned())
                        }
                    }

                    current_case_value = value;
                    debug!("current_case_value is now {current_case_value:?}");
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(mut ty) = current_type.take() {
                        ty.fields = current_field_set.take();
                        types.push(ty);
                        debug!("DONE: {types:?}");
                    }
                    in_switch = false;
                    current_case_value = None;
                } else if e.name().as_ref() == b"switch" {
                    in_switch = false;
                    debug!("Exited switch");
                } else if e.name().as_ref() == b"case" {
                    current_case_value = None;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    for protocol_type in types {
        out.push_str(&generate_type(&protocol_type));
    }

    out
}
