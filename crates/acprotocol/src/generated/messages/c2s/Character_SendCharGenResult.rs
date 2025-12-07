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

// Character creation result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SendCharGenResult")]
pub struct CharacterSendCharGenResult {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "Result")]
    pub result: CharGenResult,
}

