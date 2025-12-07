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

// Dismiss a player from the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Fellowship_Dismiss")]
pub struct FellowshipDismiss {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

