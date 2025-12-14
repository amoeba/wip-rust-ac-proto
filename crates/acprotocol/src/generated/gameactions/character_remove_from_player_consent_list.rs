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

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_RemoveFromPlayerConsentList")]
pub struct CharacterRemoveFromPlayerConsentList {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl crate::readers::ACDataType for CharacterRemoveFromPlayerConsentList {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

