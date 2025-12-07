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

// Adds an object to the trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_AddToTrade")]
pub struct TradeAddToTrade {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
}

