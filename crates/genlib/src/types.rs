use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) field_type: String,
}

#[derive(Debug)]
pub struct FieldSet {
    /// The field being switched on, if any (e.g., "Type")
    pub(crate) switch_field: Option<String>,
    /// Fields before the switch (or all fields if no switch)
    pub(crate) common_fields: Vec<Field>,
    /// Per-variant fields, keyed by case value. None if no switch.
    pub(crate) variant_fields: Option<HashMap<String, Vec<Field>>>,
}

#[derive(Debug)]
pub struct ProtocolType {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) fields: Option<FieldSet>,
}
