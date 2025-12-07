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

// Admin Returns plugin info
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Admin_QueryPluginResponse")]
pub struct AdminQueryPluginResponse {
    #[serde(rename = "Context")]
    pub context: u32,
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "PluginName")]
    pub plugin_name: String,
    #[serde(rename = "PluginAuthor")]
    pub plugin_author: String,
    #[serde(rename = "PluginEmail")]
    pub plugin_email: String,
    #[serde(rename = "PluginWebpage")]
    pub plugin_webpage: String,
}

