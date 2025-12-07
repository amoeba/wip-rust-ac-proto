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

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ObjDescEvent")]
pub struct ItemObjDescEvent {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "InstanceSequence")]
    pub instance_sequence: u16,
    #[serde(rename = "VisualDescSequence")]
    pub visual_desc_sequence: u16,
}

