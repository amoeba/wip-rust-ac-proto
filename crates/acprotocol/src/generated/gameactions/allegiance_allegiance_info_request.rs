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

// Request allegiance info for a player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceInfoRequest")]
pub struct AllegianceAllegianceInfoRequest {
    #[serde(rename = "TargetName")]
    pub target_name: String,
}

impl crate::readers::ACDataType for AllegianceAllegianceInfoRequest {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let target_name = read_string(reader)?;

        Ok(Self {
            target_name,
        })
    }
}

