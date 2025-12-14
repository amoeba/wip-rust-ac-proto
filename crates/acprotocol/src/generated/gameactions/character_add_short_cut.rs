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

// Add an item to the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddShortCut")]
pub struct CharacterAddShortCut {
    #[serde(rename = "Shortcut")]
    pub shortcut: ShortCutData,
}

impl crate::readers::ACDataType for CharacterAddShortCut {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let shortcut = ShortCutData::read(reader)?;

        Ok(Self {
            shortcut,
        })
    }
}

