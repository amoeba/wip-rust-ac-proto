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

// Spend XP to raise an attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute")]
pub struct TrainTrainAttribute {
    #[serde(rename = "Type")]
    pub type_: AttributeId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

