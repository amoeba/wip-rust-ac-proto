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

// Update Rent Payment
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_UpdateRentPayment")]
pub struct HouseUpdateRentPayment {
    #[serde(rename = "Rent")]
    pub rent: PackableList<HousePayment>,
}

