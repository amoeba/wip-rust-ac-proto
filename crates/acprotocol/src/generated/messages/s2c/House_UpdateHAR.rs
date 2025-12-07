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

// House Guest List
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateHAR")]
pub struct HouseUpdateHAR {
    #[serde(rename = "GuestList")]
    pub guest_list: HAR,
}

