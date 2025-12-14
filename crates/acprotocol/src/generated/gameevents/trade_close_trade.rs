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

// CloseTrade: End trading
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Trade_CloseTrade")]
pub struct TradeCloseTrade {
    #[serde(rename = "Reason")]
    pub reason: EndTradeReason,
}

impl crate::readers::ACDataType for TradeCloseTrade {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let reason = EndTradeReason::try_from(read_u32(reader)?)?;

        Ok(Self {
            reason,
        })
    }
}

