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

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_ServerSaysMoveItem")]
pub struct ItemServerSaysMoveItem {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

