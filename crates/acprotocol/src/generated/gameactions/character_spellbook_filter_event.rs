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

// Changes the spell book filter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SpellbookFilterEvent")]
pub struct CharacterSpellbookFilterEvent {
    #[serde(rename = "Options")]
    pub options: SpellBookFilterOptions,
}

impl crate::readers::ACDataType for CharacterSpellbookFilterEvent {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let options = Ok::<_, Box<dyn std::error::Error>>(SpellBookFilterOptions::from_bits_retain(read_u32(reader)?))?;

        Ok(Self {
            options,
        })
    }
}

