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

// Sends an abuse report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Character_AbuseLogRequest")]
pub struct CharacterAbuseLogRequest {
    #[serde(rename = "Character")]
    pub character: String,
    #[serde(rename = "Status")]
    pub status: u32,
    #[serde(rename = "Complaint")]
    pub complaint: String,
}

