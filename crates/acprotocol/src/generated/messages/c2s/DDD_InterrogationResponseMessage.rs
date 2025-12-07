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

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_InterrogationResponseMessage")]
pub struct DDDInterrogationResponseMessage {
    #[serde(rename = "Language")]
    pub language: u32,
    #[serde(rename = "Files")]
    pub files: PackableList<long>,
}

