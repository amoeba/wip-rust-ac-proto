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

// Merges one stack with another
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableMerge")]
pub struct InventoryStackableMerge {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "TargetId")]
    pub target_id: ObjectId,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

