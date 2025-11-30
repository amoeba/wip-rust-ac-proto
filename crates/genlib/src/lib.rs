use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};

#[derive(Debug)]
struct Field {
    name: String,
    ty: String,
}

#[derive(Debug)]
struct SchemaType {
    name: String,
    text: Option<String>,
    fields: Vec<Field>,
}

fn generate_type(schema_type: &SchemaType) -> String {
    let type_name = &schema_type.name;
    println!("generate type name = {type_name}");

    let mut out = String::new();

    if let Some(text_str) = &schema_type.text {
        out.push_str(format!("// {text_str}\n").as_str());
    }

    // fields
    println!("FIELDS");
    let mut fields: Vec<String> = Vec::new();

    for field in &schema_type.fields {
        println!("FIELD");

        println!("{}", field.name);
        fields.push(format!("    {}: String", field.name));
    }
    let fields_out = fields.join(",\n");

    out.push_str(
        format!(
            "#[derive(Debug, Clone)]
pub struct {type_name} {{
{fields_out}
}}\n\n",
        )
        .as_str(),
    );

    out
}

pub fn generate(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    // TODO: Use a better data structure
    let mut out = String::new();

    let mut types: Vec<SchemaType> = Vec::new();
    let mut current_type: Option<SchemaType> = None;

    loop {
        println!("loop it, current_type={current_type:?}");

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
                    println!("Resetting current_type to {:?}", name);
                    current_type = Some(SchemaType {
                        name: name.unwrap_or_default(),
                        text,
                        fields: Vec::new(),
                    });
                } else if tag_name == "field" {
                    if let Some(ref mut ty) = current_type {
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

                        if let (Some(fname), Some(ftype)) = (field_name, field_type) {
                            ty.fields.push(Field {
                                name: fname,
                                ty: ftype,
                            });
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                // closing </type>
                if e.name().as_ref() == b"type" {
                    if let Some(ty) = current_type.take() {
                        types.push(ty);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    print!("ALL TYPES: {types:?}");

    for schema_type in types {
        out.push_str(&generate_type(&schema_type));
    }

    out
}
