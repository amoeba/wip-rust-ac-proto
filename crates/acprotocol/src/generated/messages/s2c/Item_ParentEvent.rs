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

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ParentEvent")]
pub struct ItemParentEvent {
    #[serde(rename = "ParentId")]
    pub parent_id: ObjectId,
    #[serde(rename = "ChildId")]
    pub child_id: ObjectId,
    #[serde(rename = "Location")]
    pub location: ParentLocation,
    #[serde(rename = "Placement")]
    pub placement: Placement,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ChildPositionSequence")]
    pub child_position_sequence: u16,
}

