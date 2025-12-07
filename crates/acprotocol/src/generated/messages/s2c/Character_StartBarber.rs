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

// Opens barber UI
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_StartBarber")]
pub struct CharacterStartBarber {
    #[serde(rename = "BasePalette")]
    pub base_palette: DataId,
    #[serde(rename = "HeadObject")]
    pub head_object: DataId,
    #[serde(rename = "HeadTexture")]
    pub head_texture: DataId,
    #[serde(rename = "DefaultHeadTexture")]
    pub default_head_texture: DataId,
    #[serde(rename = "EyesTexture")]
    pub eyes_texture: DataId,
    #[serde(rename = "DefaultEyesTexture")]
    pub default_eyes_texture: DataId,
    #[serde(rename = "NoseTexture")]
    pub nose_texture: DataId,
    #[serde(rename = "DefaultNoseTexture")]
    pub default_nose_texture: DataId,
    #[serde(rename = "MouthTexture")]
    pub mouth_texture: DataId,
    #[serde(rename = "DefaultMouthTexture")]
    pub default_mouth_texture: DataId,
    #[serde(rename = "SkinPalette")]
    pub skin_palette: DataId,
    #[serde(rename = "HairPalette")]
    pub hair_palette: DataId,
    #[serde(rename = "EyesPalette")]
    pub eyes_palette: DataId,
    #[serde(rename = "SetupId")]
    pub setup_id: DataId,
    #[serde(rename = "Option1")]
    pub option1: i32,
    #[serde(rename = "Option2")]
    pub option2: i32,
}

