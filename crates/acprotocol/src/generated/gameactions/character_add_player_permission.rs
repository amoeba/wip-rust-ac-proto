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

// Grants a player corpse looting permission, /permit add
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AddPlayerPermission")]
pub struct CharacterAddPlayerPermission {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl crate::readers::ACDataType for CharacterAddPlayerPermission {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

