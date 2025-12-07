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

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Communication_ChatRoomTracker")]
pub struct CommunicationChatRoomTracker {
    #[serde(rename = "AllegianceRoomId")]
    pub allegiance_room_id: u32,
    #[serde(rename = "GeneralChatRoomId")]
    pub general_chat_room_id: u32,
    #[serde(rename = "TradeChatRoomId")]
    pub trade_chat_room_id: u32,
    #[serde(rename = "LFGChatRoomId")]
    pub lfg_chat_room_id: u32,
    #[serde(rename = "RoleplayChatRoomId")]
    pub roleplay_chat_room_id: u32,
    #[serde(rename = "OlthoiChatRoomId")]
    pub olthoi_chat_room_id: u32,
    #[serde(rename = "SocietyChatRoomId")]
    pub society_chat_room_id: u32,
    #[serde(rename = "SocietyCelestialHandChatRoomId")]
    pub society_celestial_hand_chat_room_id: u32,
    #[serde(rename = "SocietyEldrichWebChatRoomId")]
    pub society_eldrich_web_chat_room_id: u32,
    #[serde(rename = "SocietyRadiantBloodChatRoomId")]
    pub society_radiant_blood_chat_room_id: u32,
}

