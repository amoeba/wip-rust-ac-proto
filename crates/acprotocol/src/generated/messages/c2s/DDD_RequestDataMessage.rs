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

// DDD request for data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_RequestDataMessage")]
pub struct DDDRequestDataMessage {
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
}

