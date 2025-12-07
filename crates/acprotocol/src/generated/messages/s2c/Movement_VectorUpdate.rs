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

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Movement_VectorUpdate")]
pub struct MovementVectorUpdate {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Velocity")]
    pub velocity: Vector3,
    #[serde(rename = "Omega")]
    pub omega: Vector3,
    #[serde(rename = "ObjectInstanceSequence")]
    pub object_instance_sequence: u16,
    #[serde(rename = "ObjectVectorSequence")]
    pub object_vector_sequence: u16,
}

