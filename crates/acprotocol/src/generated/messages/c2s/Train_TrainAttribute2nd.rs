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

// Spend XP to raise a vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Train_TrainAttribute2nd")]
pub struct TrainTrainAttribute2nd {
    #[serde(rename = "Type")]
    pub type_: VitalId,
    #[serde(rename = "Experience")]
    pub experience: u32,
}

