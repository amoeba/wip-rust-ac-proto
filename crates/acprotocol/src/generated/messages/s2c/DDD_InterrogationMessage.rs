use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationMessage")]
pub struct DDDInterrogationMessage {
    #[serde(rename = "ServersRegion")]
    pub servers_region: u32,
    #[serde(rename = "NameRuleLanguage")]
    pub name_rule_language: u32,
    #[serde(rename = "ProductId")]
    pub product_id: u32,
    #[serde(rename = "SupportedLanguages")]
    pub supported_languages: PackableList<uint>,
}

