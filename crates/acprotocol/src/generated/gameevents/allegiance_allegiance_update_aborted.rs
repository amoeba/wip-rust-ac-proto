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

// Allegiance update cancelled
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Allegiance_AllegianceUpdateAborted")]
pub struct AllegianceAllegianceUpdateAborted {
    #[serde(rename = "FailureType")]
    pub failure_type: WeenieError,
}

impl crate::readers::ACDataType for AllegianceAllegianceUpdateAborted {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let failure_type = WeenieError::try_from(read_u32(reader)?)?;

        Ok(Self {
            failure_type,
        })
    }
}

