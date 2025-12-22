/// Macro to extract XML attributes into Option variables
///
/// Usage:
/// ```ignore
/// extract_xml_attrs!(element, {
///     b"type" => my_type,
///     b"name" => my_name,
///     b"length" => my_length
/// });
/// ```
///
/// This will create `my_type`, `my_name`, and `my_length` as `Option<String>` variables
/// and populate them from the XML attributes if present.
#[macro_export]
macro_rules! extract_xml_attrs {
    ($element:expr, { $($attr_name:expr => $var:ident),* $(,)? }) => {
        $(let mut $var = None;)*
        for attr in $element.attributes().flatten() {
            match attr.key.as_ref() {
                $($attr_name => $var = Some(attr.unescape_value().unwrap().into_owned()),)*
                _ => {}
            }
        }
    };
}

pub mod tags {
    pub mod process_align_tag;
    pub mod process_case_tag;
    pub mod process_enum_start_tag;
    pub mod process_enum_value_tag;
    pub mod process_field_tag;
    pub mod process_subfield_tag;
    pub mod process_switch_tag;
    pub mod process_table_tag;
    pub mod process_type_tag;
    pub mod process_vector_tag;
}

pub mod utils {
    pub mod add_field_to_set;
    pub mod create_field_from_tag;

    // Re-export the main routing function for convenience
    pub use add_field_to_set::route_field;
}

#[cfg(test)]
mod tests {
    use quick_xml::events::BytesStart;

    #[test]
    fn test_extract_xml_attrs_all_present() {
        let xml = r#"<field type="u32" name="count" param="test"/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type,
            b"name" => field_name,
            b"param" => param
        });

        assert_eq!(field_type, Some("u32".to_string()));
        assert_eq!(field_name, Some("count".to_string()));
        assert_eq!(param, Some("test".to_string()));
    }

    #[test]
    fn test_extract_xml_attrs_some_missing() {
        let xml = r#"<field type="u32" name="count"/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type,
            b"name" => field_name,
            b"param" => param,
            b"length" => length
        });

        assert_eq!(field_type, Some("u32".to_string()));
        assert_eq!(field_name, Some("count".to_string()));
        assert_eq!(param, None);
        assert_eq!(length, None);
    }

    #[test]
    fn test_extract_xml_attrs_all_missing() {
        let xml = r#"<field other="value"/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type,
            b"name" => field_name
        });

        assert_eq!(field_type, None);
        assert_eq!(field_name, None);
    }

    #[test]
    fn test_extract_xml_attrs_empty() {
        let xml = r#"<field/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type
        });

        assert_eq!(field_type, None);
    }

    #[test]
    fn test_extract_xml_attrs_with_special_characters() {
        let xml = r#"<field type="Vec&lt;u32&gt;" name="my_field"/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type,
            b"name" => field_name
        });

        // XML escaping should be handled by unescape_value()
        assert_eq!(field_type, Some("Vec<u32>".to_string()));
        assert_eq!(field_name, Some("my_field".to_string()));
    }

    #[test]
    fn test_extract_xml_attrs_single_attribute() {
        let xml = r#"<align type="DWORD"/>"#;
        let element = BytesStart::from_content(xml, "align".len());

        crate::extract_xml_attrs!(element, {
            b"type" => align_type
        });

        assert_eq!(align_type, Some("DWORD".to_string()));
    }

    #[test]
    fn test_extract_xml_attrs_trailing_comma() {
        let xml = r#"<vector type="u8" name="data" length="10"/>"#;
        let element = BytesStart::from_content(xml, "vector".len());

        // Test that trailing comma is allowed
        crate::extract_xml_attrs!(element, {
            b"type" => vector_type,
            b"name" => vector_name,
            b"length" => length,
        });

        assert_eq!(vector_type, Some("u8".to_string()));
        assert_eq!(vector_name, Some("data".to_string()));
        assert_eq!(length, Some("10".to_string()));
    }

    #[test]
    fn test_extract_xml_attrs_duplicate_attributes_first_wins() {
        // Quick-xml handles duplicates by returning both, but our macro will
        // overwrite the value on each match, so the first one that appears wins
        let xml = r#"<field type="u32" type="u64"/>"#;
        let element = BytesStart::from_content(xml, "field".len());

        crate::extract_xml_attrs!(element, {
            b"type" => field_type
        });

        // The first attribute value is used
        assert_eq!(field_type, Some("u32".to_string()));
    }

    #[test]
    fn test_extract_xml_attrs_table_tag() {
        let xml = r#"<table name="items" key="u32" value="Item" length="Count"/>"#;
        let element = BytesStart::from_content(xml, "table".len());

        crate::extract_xml_attrs!(element, {
            b"name" => table_name,
            b"key" => key_type,
            b"value" => value_type,
            b"length" => length_expr
        });

        assert_eq!(table_name, Some("items".to_string()));
        assert_eq!(key_type, Some("u32".to_string()));
        assert_eq!(value_type, Some("Item".to_string()));
        assert_eq!(length_expr, Some("Count".to_string()));
    }
}
