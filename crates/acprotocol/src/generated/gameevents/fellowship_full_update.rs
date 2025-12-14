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

// Create or join a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_FullUpdate")]
pub struct FellowshipFullUpdate {
    #[serde(rename = "Fellowship")]
    pub fellowship: Fellowship,
}

impl crate::readers::ACDataType for FellowshipFullUpdate {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let fellowship = Fellowship::read(reader)?;

        Ok(Self {
            fellowship,
        })
    }
}

