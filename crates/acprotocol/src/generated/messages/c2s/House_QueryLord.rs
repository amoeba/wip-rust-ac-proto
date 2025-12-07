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

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "House_QueryLord")]
pub struct HouseQueryLord {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
}

