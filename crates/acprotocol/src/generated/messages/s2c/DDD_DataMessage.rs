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

// Add or update a dat file Resource.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDDataMessageType0 {
    #[serde(rename = "DatFile")]
    pub dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "Iteration")]
    pub iteration: u32,
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "DataSize")]
    pub data_size: u32,
    #[serde(rename = "Data")]
    pub data: Vec<byte>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDDDataMessageType1 {
    #[serde(rename = "DatFile")]
    pub dat_file: DatFileType,
    #[serde(rename = "ResourceType")]
    pub resource_type: u32,
    #[serde(rename = "ResourceId")]
    pub resource_id: DataId,
    #[serde(rename = "Iteration")]
    pub iteration: u32,
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "DataSize")]
    pub data_size: u32,
    #[serde(rename = "FileSize")]
    pub file_size: u32,
    pub data: Vec<byte>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "DDD_DataMessage")]
#[serde(tag = "Compression")]
pub enum DDDDataMessage {
    #[serde(rename = "0x00")]
    Type0(DDDDataMessageType0),
    #[serde(rename = "0x01")]
    Type1(DDDDataMessageType1),
}

