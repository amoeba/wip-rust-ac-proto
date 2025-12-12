use log::debug;

use crate::util::parse_enum_value;

/// Process a case tag in the XML
pub fn process_case_tag(e: &quick_xml::events::BytesStart) -> Option<Vec<i64>> {
    let mut value = None;

    for attr in e.attributes().flatten() {
        if attr.key.as_ref() == b"value" {
            value = Some(attr.unescape_value().unwrap().into_owned())
        }
    }

    // Parse multi-value cases (e.g., "0x01 | 0x08 | 0x0A") into individual parsed i64 values
    let values = value.map(|v| {
        v.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|s| {
                parse_enum_value(s)
                    .map_err(|e| {
                        eprintln!("Warning: {}", e);
                        e
                    })
                    .ok()
            })
            .collect::<Vec<i64>>()
    });

    debug!("current_case_value is now {values:?}");
    values
}
