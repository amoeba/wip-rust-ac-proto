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

// You have been portal stormed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStorm")]
pub struct MiscPortalStorm {}

