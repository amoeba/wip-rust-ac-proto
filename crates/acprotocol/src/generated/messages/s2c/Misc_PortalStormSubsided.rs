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

// The portal storm has subsided.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormSubsided")]
pub struct MiscPortalStormSubsided {}

