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

// Create an object somewhere in the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_CreateObject")]
pub struct ItemCreateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
    #[serde(rename = "PhysicsDescription")]
    pub physics_description: PhysicsDesc,
    #[serde(rename = "WeenieDescription")]
    pub weenie_description: PublicWeenieDesc,
}

