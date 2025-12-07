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

// Lists available house /house available
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_ListAvailableHouses")]
pub struct HouseListAvailableHouses {
    #[serde(rename = "Type")]
    pub type_: HouseType,
}

