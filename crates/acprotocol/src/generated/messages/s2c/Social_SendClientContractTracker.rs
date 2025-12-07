use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::readers::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use crate::enums::*;

// Updates a contract data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_SendClientContractTracker")]
pub struct SocialSendClientContractTracker {
    #[serde(rename = "ContractTracker")]
    pub contract_tracker: ContractTracker,
    #[serde(rename = "DeleteContract")]
    pub delete_contract: bool,
    #[serde(rename = "SetAsDisplayContract")]
    pub set_as_display_contract: bool,
}

