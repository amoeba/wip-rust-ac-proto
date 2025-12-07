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

// A portal storm is brewing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Misc_PortalStormBrewing")]
pub struct MiscPortalStormBrewing {
    #[serde(rename = "Extent")]
    pub extent: f32,
}

