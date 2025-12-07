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

// Teleports player to their allegiance housing, /house mansion_recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_TeleToMansion")]
pub struct HouseTeleToMansion {}

