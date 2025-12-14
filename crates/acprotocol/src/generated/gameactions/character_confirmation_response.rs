use serde::{Serialize, Deserialize};
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

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

impl crate::readers::ACDataType for CharacterConfirmationResponse {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let type_ = ConfirmationType::try_from(read_u32(reader)?)?;
        let context = read_u32(reader)?;
        let accepted = read_bool(reader)?;

        Ok(Self {
            type_,
            context,
            accepted,
        })
    }
}

