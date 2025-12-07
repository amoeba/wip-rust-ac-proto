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

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "Writing_BookDeletePageResponse")]
pub struct WritingBookDeletePageResponse {
    #[serde(rename = "BookId")]
    pub book_id: ObjectId,
    #[serde(rename = "PageNumber")]
    pub page_number: u32,
    #[serde(rename = "Success")]
    pub success: bool,
}

