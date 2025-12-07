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

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_SetPlayerVisualDesc")]
pub struct CharacterSetPlayerVisualDesc {
    #[serde(rename = "ObjectDescription")]
    pub object_description: ObjDesc,
}

