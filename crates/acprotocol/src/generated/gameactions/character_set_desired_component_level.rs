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

// Sets a new fill complevel for a component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetDesiredComponentLevel")]
pub struct CharacterSetDesiredComponentLevel {
    #[serde(rename = "Wcid")]
    pub wcid: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl crate::readers::ACDataType for CharacterSetDesiredComponentLevel {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let wcid = read_u32(reader)?;
        let amount = read_u32(reader)?;

        Ok(Self {
            wcid,
            amount,
        })
    }
}

