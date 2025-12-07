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

// A list of dat files that need to be patched
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_BeginDDDMessage")]
pub struct DDDBeginDDDMessage {
    #[serde(rename = "DataExpected")]
    pub data_expected: u32,
    #[serde(rename = "Revisions")]
    pub revisions: PackableList<DDDRevision>,
}

