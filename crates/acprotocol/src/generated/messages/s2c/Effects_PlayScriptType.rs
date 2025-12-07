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

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptType")]
pub struct EffectsPlayScriptType {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptType")]
    pub script_type: i32,
    #[serde(rename = "Speed")]
    pub speed: f32,
}

