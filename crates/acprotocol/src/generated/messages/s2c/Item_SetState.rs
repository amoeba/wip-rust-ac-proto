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

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetState")]
pub struct ItemSetState {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "NewState")]
    pub new_state: PhysicsState,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectStateSequence")]
    pub object_state_sequence: u16,
}

