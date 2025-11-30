use quick_xml::{
    Reader,
    events::{BytesStart, Event},
};

fn gen_foo(xml: &str) -> String {
    format!(
        "// comment

pub fn dummy_from_schema() -> String {{
    format!(\"len is {}\")
}}
",
        xml.len()
    )
}

fn generate_type(e: &BytesStart) -> String {
    println!("generate_type");

    let mut name = String::new();
    let mut comment = String::new();

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"name" {
            let value = attr.unescape_value().unwrap();
            name = value.to_string();
        } else if attr.key.as_ref() == b"text" {
            let value = attr.unescape_value().unwrap();
            comment = value.to_string();
        }
    }

    println!("type name = {name}");

    format!(
        "// comment: {comment}
#[derive(Debug, Clone)]
pub struct {name} {{

}}
    "
    )
}

// fn generate_enum() -> String {
//     format!()
// }

pub fn generate(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    // TODO: Use a better data structure
    let mut out = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name: &[u8] = e.name().0;
                if name == b"type" {
                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"name" {
                            let value = attr.unescape_value().unwrap();
                            println!("type name = {value}");

                            out.push_str(&generate_type(&e));
                        }
                    }

                    break;
                }

                // if name == b"enum" {
                //     for attr in e.attributes().flatten() {
                //         if attr.key.as_ref() == b"name" {
                //             let value = attr.unescape_value().unwrap();
                //             println!("enum name = {value}");
                //         }
                //     }
                // }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("error at position {}: {}", reader.buffer_position(), e),
            _ => {}
        }
        buf.clear();
    }

    out
}
