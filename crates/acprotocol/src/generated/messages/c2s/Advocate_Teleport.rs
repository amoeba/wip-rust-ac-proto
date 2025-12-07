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

// Advocate Teleport
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Advocate_Teleport")]
pub struct AdvocateTeleport {
    #[serde(rename = "ObjectId")]
    pub object_id: String,
    #[serde(rename = "Destination")]
    pub destination: Position,
}

