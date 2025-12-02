use core::panic;
use std::collections::HashMap;

use log::debug;
use quick_xml::{Reader, events::Event};

use crate::{
    types::{Field, FieldSet, ProtocolType, VariantFieldSet},
    util::safe_field_name,
};

mod types;
mod util;

fn generate_type(protocol_type: &ProtocolType) -> String {
    let type_name = &protocol_type.name;
    println!("generate_type: name = {type_name}");

    let mut out = String::new();

    if let Some(text_str) = &protocol_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }
    let mut field_out: Vec<String> = Vec::new();

    if let Some(field_set) = &protocol_type.fields {
        match field_set {
            FieldSet::SimpleFieldSet(fields) => {
                for field in fields {
                    let original_name = &field.name;
                    let (safe_name, needs_rename) = safe_field_name(original_name);

                    if needs_rename {
                        field_out.push(format!(
                            "    #[serde(rename = \"{original_name}\")]\n    {safe_name}: String"
                        ));
                    } else {
                        field_out.push(format!("    {safe_name}: String"));
                    }
                }
            }
            FieldSet::VariantFieldSet(variant_field_set) => {
                // TODO: This is actually just the above code and I did this just so we can compile
                for (name, fields) in &variant_field_set.fields {
                    for field in fields {
                        let original_name = &field.name;
                        let (safe_name, needs_rename) = safe_field_name(original_name);

                        if needs_rename {
                            field_out.push(format!(
                            "    #[serde(rename = \"{original_name}\")]\n    {safe_name}: String"
                        ));
                        } else {
                            field_out.push(format!("    {safe_name}: String"));
                        }
                    }
                }
            }
        }
    }
    let fields_out: String = field_out.join(",\n");

    out.push_str(
        format!(
            "#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct {type_name} {{
{fields_out}
}}\n\n",
        )
        .as_str(),
    );

    out
}

pub fn generate(xml: &str, filter_types: Option<Vec<String>>) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    // TODO: Use a better data structure?
    let mut out = String::new();

    let mut types: Vec<ProtocolType> = Vec::new();
    let mut current_type: Option<ProtocolType> = None;
    // Not 100% sure I need these two since we have current_type
    let mut current_variant_field_id: Option<String> = None;
    let mut current_variant_fieldset: Option<VariantFieldSet> = None;

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

                    if let (Some(name)) = name {
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

                        debug!("current_type is now {current_type:?}");
                    }
                } else if tag_name == "field" {
                    let mut field_type = None;
                    let mut field_name = None;
                    let mut field_text = None;

                    for attr in e.attributes().flatten() {
                        match attr.key.as_ref() {
                            b"type" => {
                                field_type = Some(attr.unescape_value().unwrap().into_owned())
                            }
                            b"name" => {
                                field_name = Some(attr.unescape_value().unwrap().into_owned())
                            }
                            b"text" => {
                                field_text = Some(attr.unescape_value().unwrap().into_owned());
                            }
                            _ => {}
                        }
                    }

                    debug!("Processing field {field_name:?}");

                    if let (Some(fname), Some(ftype)) = (field_name, field_type) {
                        // Handle VariantFieldSet first
                        if let Some(ref mut var) = current_variant_fieldset {
                            debug!("Processing field as variant field...");
                            let new_field = Field {
                                name: fname,
                                field_type: ftype,
                            };

                            if let Some(ref id) = current_variant_field_id {
                                let variant_fieldset =
                                    var.fields.entry(id.to_string()).or_insert(Vec::new());
                                variant_fieldset.push(new_field);
                            }
                        }
                        // SimpleFieldSet behavior below
                        else if let Some(ref mut ty) = current_type {
                            debug!("Processing field as normal field...");
                            let fields = ty
                                .fields
                                .get_or_insert_with(|| FieldSet::SimpleFieldSet(Vec::new()));
                            if let FieldSet::SimpleFieldSet(vec) = fields {
                                vec.push(Field {
                                    name: fname,
                                    field_type: ftype,
                                });
                            }

                            debug!("after adding fields, current_type is now {ty:?}");
                        }
                    }
                } else if tag_name == "switch" {
                    let mut name: Option<String> = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            name = Some(attr.unescape_value().unwrap().into_owned())
                        }
                    }

                    // Set a variant so we accumulate fields into the variant for now
                    if let Some(name) = name {
                        current_variant_fieldset = Some(VariantFieldSet {
                            name,
                            fields: HashMap::new(),
                        });
                    }

                    debug!("current_variant_fieldset is now {current_variant_fieldset:?}");
                } else if tag_name == "case" {
                    let mut value = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"value" {
                            value = Some(attr.unescape_value().unwrap().into_owned())
                        }
                    }

                    debug!("TODO: Need to handle <case> element");
                    current_variant_field_id = value;
                    debug!("current_variant_field_id is now {current_variant_field_id:?}");
                    // if let (Some(val), Some(ref mut current)) = (value, current_variant_fieldset) {
                    //     current.fields.insert(val, Vec::new());
                    // }
                }
            }
            Ok(Event::End(e)) => {
                // closing </type>
                if e.name().as_ref() == b"type" {
                    // Close out type
                    if let Some(ty) = current_type.take() {
                        types.push(ty);
                        debug!("DONE: {types:?}");
                    }
                } else if e.name().as_ref() == b"switch" {
                    // Set current variant on current type and reset current_variant_fieldset
                    if let (Some(ty), Some(variant)) =
                        (&mut current_type, current_variant_fieldset.take())
                    {
                        ty.fields = Some(FieldSet::VariantFieldSet(variant));
                    }
                } else if e.name().as_ref() == b"case" {
                    // TODO
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
