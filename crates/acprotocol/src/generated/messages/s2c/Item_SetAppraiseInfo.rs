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

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Item_SetAppraiseInfo")]
pub struct ItemSetAppraiseInfo {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "IntProperties")]
    pub int_properties: Option<PackableHashTable<PropertyInt, int>>,
    #[serde(rename = "Int64Properties")]
    pub int64_properties: Option<PackableHashTable<PropertyInt64, long>>,
    #[serde(rename = "BoolProperties")]
    pub bool_properties: Option<PackableHashTable<PropertyBool, bool>>,
    #[serde(rename = "FloatProperties")]
    pub float_properties: Option<PackableHashTable<PropertyFloat, double>>,
    #[serde(rename = "StringProperties")]
    pub string_properties: Option<PackableHashTable<PropertyString, string>>,
    #[serde(rename = "DataIdProperties")]
    pub data_id_properties: Option<PackableHashTable<PropertyDataId, DataId>>,
    #[serde(rename = "SpellBook")]
    pub spell_book: Option<PackableList<LayeredSpellId>>,
    #[serde(rename = "ArmorProfile")]
    pub armor_profile: Option<ArmorProfile>,
    #[serde(rename = "CreatureProfile")]
    pub creature_profile: Option<CreatureAppraisalProfile>,
    #[serde(rename = "WeaponProfile")]
    pub weapon_profile: Option<WeaponProfile>,
    #[serde(rename = "HookProfile")]
    pub hook_profile: Option<HookAppraisalProfile>,
    #[serde(rename = "ArmorHighlight")]
    pub armor_highlight: Option<ArmorHighlightMask>,
    #[serde(rename = "ArmorColor")]
    pub armor_color: Option<ArmorHighlightMask>,
    #[serde(rename = "WeaponHighlight")]
    pub weapon_highlight: Option<WeaponHighlightMask>,
    #[serde(rename = "WeaponColor")]
    pub weapon_color: Option<WeaponHighlightMask>,
    #[serde(rename = "ResistHighlight")]
    pub resist_highlight: Option<ResistHighlightMask>,
    #[serde(rename = "ResistColor")]
    pub resist_color: Option<ResistHighlightMask>,
    #[serde(rename = "BaseArmorHead")]
    pub base_armor_head: Option<u32>,
    #[serde(rename = "BaseArmorChest")]
    pub base_armor_chest: Option<u32>,
    #[serde(rename = "BaseArmorGroin")]
    pub base_armor_groin: Option<u32>,
    #[serde(rename = "BaseArmorBicep")]
    pub base_armor_bicep: Option<u32>,
    #[serde(rename = "BaseArmorWrist")]
    pub base_armor_wrist: Option<u32>,
    #[serde(rename = "BaseArmorHand")]
    pub base_armor_hand: Option<u32>,
    #[serde(rename = "BaseArmorThigh")]
    pub base_armor_thigh: Option<u32>,
    #[serde(rename = "BaseArmorShin")]
    pub base_armor_shin: Option<u32>,
    #[serde(rename = "BaseArmorFoot")]
    pub base_armor_foot: Option<u32>,
}

