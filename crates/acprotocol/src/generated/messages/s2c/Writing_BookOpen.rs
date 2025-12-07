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

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookOpen")]
pub struct WritingBookOpen {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "MaxNumPages")]
    pub max_num_pages: u32,
    #[serde(rename = "PageData")]
    pub page_data: PageDataList,
    #[serde(rename = "Inscription")]
    pub inscription: String,
    #[serde(rename = "ScribeId")]
    pub scribe_id: ObjectId,
    #[serde(rename = "ScribeName")]
    pub scribe_name: String,
}

