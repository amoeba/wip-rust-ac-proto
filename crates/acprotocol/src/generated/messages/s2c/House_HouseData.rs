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

// House panel information for owners.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_HouseData")]
pub struct HouseHouseData {
    #[serde(rename = "Data")]
    pub data: HouseData,
}

