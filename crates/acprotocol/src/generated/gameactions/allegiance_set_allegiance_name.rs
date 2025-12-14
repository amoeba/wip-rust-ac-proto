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

// Sets the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_SetAllegianceName")]
pub struct AllegianceSetAllegianceName {
    #[serde(rename = "Name")]
    pub name: String,
}

impl crate::readers::ACDataType for AllegianceSetAllegianceName {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let name = read_string(reader)?;

        Ok(Self {
            name,
        })
    }
}

