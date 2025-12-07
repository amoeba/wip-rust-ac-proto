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

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Effects_PlayScriptId")]
pub struct EffectsPlayScriptId {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "ScriptId")]
    pub script_id: DataId,
}

