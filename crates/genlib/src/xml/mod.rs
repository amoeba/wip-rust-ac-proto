pub mod tags {
    pub mod process_type_tag;
    pub mod process_enum_start_tag;
    pub mod process_enum_value_tag;
    pub mod process_field_tag;
    pub mod process_vector_tag;
    pub mod process_table_tag;
    pub mod process_switch_tag;
    pub mod process_case_tag;
    pub mod process_subfield_tag;
    pub mod process_align_tag;
}

pub mod utils {
    pub mod create_field_from_tag;
    pub mod add_field_to_set;
}