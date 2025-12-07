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

// House Profile
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseTransaction")]
pub struct HouseHouseTransaction {
    #[serde(rename = "NoticeType")]
    pub notice_type: u32,
}

