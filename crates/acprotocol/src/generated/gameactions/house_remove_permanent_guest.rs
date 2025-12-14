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

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_RemovePermanentGuest")]
pub struct HouseRemovePermanentGuest {
    #[serde(rename = "GuestName")]
    pub guest_name: String,
}

impl crate::readers::ACDataType for HouseRemovePermanentGuest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_name = read_string(reader)?;

        Ok(Self {
            guest_name,
        })
    }
}

