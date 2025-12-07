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

// Boot everyone from your house, /house boot -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_BootEveryone")]
pub struct HouseBootEveryone {}

