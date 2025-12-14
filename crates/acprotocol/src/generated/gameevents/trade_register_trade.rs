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

impl crate::readers::ACDataType for TradeRegisterTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let initiator_id = ObjectId::read(reader)?;
        let partner_id = ObjectId::read(reader)?;
        let stamp = read_i64(reader)?;

        Ok(Self {
            initiator_id,
            partner_id,
            stamp,
        })
    }
}

