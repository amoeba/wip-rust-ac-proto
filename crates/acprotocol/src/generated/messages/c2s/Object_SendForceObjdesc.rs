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

// Asks server for a new object description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Object_SendForceObjdesc")]
pub struct ObjectSendForceObjdesc {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

