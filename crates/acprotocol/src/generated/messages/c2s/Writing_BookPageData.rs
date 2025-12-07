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

// Requests data for a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookPageData")]
pub struct WritingBookPageData {
    #[serde(rename = "ObjectId")]
    pub object_id: ObjectId,
    #[serde(rename = "PageNum")]
    pub page_num: i32,
}

