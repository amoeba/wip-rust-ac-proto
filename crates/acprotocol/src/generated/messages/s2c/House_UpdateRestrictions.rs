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

// Update Restrictions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRestrictions")]
pub struct HouseUpdateRestrictions {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "SenderId")]
    pub sender_id: ObjectId,
    #[serde(rename = "Restrictions")]
    pub restrictions: RestrictionDB,
}

