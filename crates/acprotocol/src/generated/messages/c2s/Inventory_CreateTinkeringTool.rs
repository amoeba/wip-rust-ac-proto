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

// Salvages items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Inventory_CreateTinkeringTool")]
pub struct InventoryCreateTinkeringTool {
    #[serde(rename = "ToolId")]
    pub tool_id: ObjectId,
    #[serde(rename = "Items")]
    pub items: PackableList<ObjectId>,
}

