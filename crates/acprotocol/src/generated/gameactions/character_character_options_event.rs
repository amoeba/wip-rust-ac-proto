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

// Set multiple character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_CharacterOptionsEvent")]
pub struct CharacterCharacterOptionsEvent {
    #[serde(rename = "Options")]
    pub options: PlayerModule,
}

impl crate::readers::ACDataType for CharacterCharacterOptionsEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let options = PlayerModule::read(reader)?;

        Ok(Self {
            options,
        })
    }
}

