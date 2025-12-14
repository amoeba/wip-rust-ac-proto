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

// Sends all contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTrackerTable")]
pub struct SocialSendClientContractTrackerTable {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTrackerTable,
}

impl crate::readers::ACDataType for SocialSendClientContractTrackerTable {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let contract_tracker = ContractTrackerTable::read(reader)?;

        Ok(Self {
            contract_tracker,
        })
    }
}

