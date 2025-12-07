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

// Split a stack and place it into a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_StackableSplitToContainer")]
pub struct InventoryStackableSplitToContainer {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ContainerId")]
    pub container_id: ObjectId,
    #[serde(rename = "SlotIndex")]
    pub slot_index: u32,
    #[serde(rename = "Amount")]
    pub amount: u32,
}

