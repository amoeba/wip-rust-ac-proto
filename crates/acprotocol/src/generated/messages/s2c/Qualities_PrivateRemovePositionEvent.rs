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

// Remove a position property from the character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Qualities_PrivateRemovePositionEvent")]
pub struct QualitiesPrivateRemovePositionEvent {
    #[serde(rename = "Sequence")]
    pub sequence: u8,
    #[serde(rename = "Type")]
    pub type_: PropertyPosition,
}

