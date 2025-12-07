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

// Create a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Create")]
pub struct FellowshipCreate {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShareXP")]
    pub share_xp: bool,
}

