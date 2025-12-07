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

// Friends list update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Social_FriendsUpdate")]
pub struct SocialFriendsUpdate {
    #[serde(rename = "Friends")]
    pub friends: PackableList<FriendData>,
    #[serde(rename = "Type")]
    pub type_: FriendsUpdateType,
}

