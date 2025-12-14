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

// Sets your house open status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_SetOpenHouseStatus")]
pub struct HouseSetOpenHouseStatus {
    #[serde(rename = "OpenHouse")]
    pub open_house: bool,
}

impl crate::readers::ACDataType for HouseSetOpenHouseStatus {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let open_house = read_bool(reader)?;

        Ok(Self {
            open_house,
        })
    }
}

