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

// Set a single character option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_PlayerOptionChangedEvent")]
pub struct CharacterPlayerOptionChangedEvent {
    #[serde(rename = "Option")]
    pub option: CharacterOptions1,
    #[serde(rename = "Value")]
    pub value: bool,
}

impl crate::readers::ACDataType for CharacterPlayerOptionChangedEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let option = Ok::<_, Box<dyn std::error::Error>>(CharacterOptions1::from_bits_retain(read_u32(reader)?))?;
        let value = read_bool(reader)?;

        Ok(Self {
            option,
            value,
        })
    }
}

