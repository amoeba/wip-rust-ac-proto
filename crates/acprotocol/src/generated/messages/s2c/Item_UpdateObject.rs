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

// Update an existing object's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_UpdateObject")]
pub struct ItemUpdateObject {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ObjectDesc")]
    pub object_desc: ObjDesc,
    #[serde(rename = "PhysicsDesc")]
    pub physics_desc: PhysicsDesc,
    #[serde(rename = "WeenieDesc")]
    pub weenie_desc: PublicWeenieDesc,
}

