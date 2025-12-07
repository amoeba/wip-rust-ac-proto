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

// Confirms a dialog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_ConfirmationResponse")]
pub struct CharacterConfirmationResponse {
    #[serde(rename = "Type")]
    pub type_: ConfirmationType,
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Accepted")]
    pub accepted: bool,
}

