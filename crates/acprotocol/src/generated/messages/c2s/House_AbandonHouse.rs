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

// Abandons your house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_AbandonHouse")]
pub struct HouseAbandonHouse {}

