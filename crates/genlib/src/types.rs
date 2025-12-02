use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) field_type: String,
}

pub type SimpleFieldSet = Vec<Field>;

#[derive(Debug)]
pub enum FieldSet {
    SimpleFieldSet(Vec<Field>),
    VariantFieldSet(VariantFieldSet),
}

#[derive(Debug)]
pub struct VariantFieldSet {
    pub(crate) name: String,
    pub(crate) fields: HashMap<String, Vec<Field>>,
}

#[derive(Debug)]
pub struct ProtocolType {
    pub(crate) name: String,
    pub(crate) text: Option<String>,
    pub(crate) fields: Option<FieldSet>,
}
