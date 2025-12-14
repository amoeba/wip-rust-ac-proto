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

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
    #[serde(rename = "GuestList")]
    pub guest_list: HAR,
}

impl crate::readers::ACDataType for HouseUpdateHAR {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let guest_list = HAR::read(reader)?;

        Ok(Self {
            guest_list,
        })
    }
}

