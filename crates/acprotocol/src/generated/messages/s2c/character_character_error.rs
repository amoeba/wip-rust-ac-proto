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

// Failure to log in
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterError")]
pub struct CharacterCharacterError {
    #[serde(rename = "Reason")]
    pub reason: CharacterErrorType,
}

impl crate::readers::ACDataType for CharacterCharacterError {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = CharacterErrorType::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

