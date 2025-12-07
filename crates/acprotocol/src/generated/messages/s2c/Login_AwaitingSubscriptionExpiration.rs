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

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Login_AwaitingSubscriptionExpiration")]
pub struct LoginAwaitingSubscriptionExpiration {
    #[serde(rename = "SecondsRemaining")]
    pub seconds_remaining: u32,
}

