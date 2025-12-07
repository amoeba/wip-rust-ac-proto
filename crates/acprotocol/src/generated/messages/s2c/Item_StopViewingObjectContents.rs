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

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_StopViewingObjectContents")]
pub struct ItemStopViewingObjectContents {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

