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

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_RegisterTrade")]
pub struct TradeRegisterTrade {
    #[serde(rename = "InitiatorId")]
    pub initiator_id: ObjectId,
    #[serde(rename = "PartnerId")]
    pub partner_id: ObjectId,
    #[serde(rename = "Stamp")]
    pub stamp: i64,
}

