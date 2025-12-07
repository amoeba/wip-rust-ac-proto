// Binary readers for c2s types

#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use crate::readers::ACReader;
#[allow(unused_imports)]
use crate::types::*;
use crate::types::c2s::*;
#[allow(unused_imports)]
use crate::enums::*;
#[allow(unused_imports)]
use super::*;

impl CommunicationTurbineChatType1 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType1BlobDispatchTypeVariant::read(reader)?;

impl CommunicationTurbineChatType1BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x01 => {
                let room_id = read_u32(reader)?;
                let display_name = read_wstring(reader).map(WString)?;
                let text = read_wstring(reader).map(WString)?;
                let extra_data_size = read_u32(reader)?;
                let speaker_id = ObjectId::read(reader)?;
                let h_result = read_i32(reader)?;
                let chat_type = read_u32(reader)?;
                return Ok(Self::Type1 {
                    room_id,
                    display_name,
                    text,
                    extra_data_size,
                    speaker_id,
                    h_result,
                    chat_type,
                });
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChatType3 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType3BlobDispatchTypeVariant::read(reader)?;

impl CommunicationTurbineChatType3BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x02 => {
                let context_id = read_u32(reader)?;
                let response_id = read_u32(reader)?;
                let method_id = read_u32(reader)?;
                let room_id = read_u32(reader)?;
                let text = read_wstring(reader).map(WString)?;
                let extra_data_size = read_u32(reader)?;
                let speaker_id = ObjectId::read(reader)?;
                let h_result = read_i32(reader)?;
                let chat_type = read_u32(reader)?;
                return Ok(Self::Type2 {
                    context_id,
                    response_id,
                    method_id,
                    room_id,
                    text,
                    extra_data_size,
                    speaker_id,
                    h_result,
                    chat_type,
                });
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChatType5 {
    pub fn read(reader: &mut dyn ACReader, mmessage_size: uint, target_type: int, target_id: int, transport_type: int, transport_id: int, cookie: int, payload_size: uint) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = CommunicationTurbineChatType5BlobDispatchTypeVariant::read(reader)?;

impl CommunicationTurbineChatType5BlobDispatchTypeVariant {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let blob_dispatch_type = read_u8(reader)?;

        match blob_dispatch_type {
            0x01 | 0x02 => {
                let context_id = read_u32(reader)?;
                let response_id = read_u32(reader)?;
                let method_id = read_u32(reader)?;
                let h_result = read_i32(reader)?;
                return Ok(Self::Type1 {
                    context_id,
                    response_id,
                    method_id,
                    h_result,
                });
            },
            _ => Err("Unknown nested switch value".into()),
        }
    }
}



        Ok(Self {
            mmessage_size,
            target_type,
            target_id,
            transport_type,
            transport_id,
            cookie,
            payload_size,
            blob_dispatch_type,
        })
    }
}

impl CommunicationTurbineChat {
    pub fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        let mmessage_size = read_u32(reader)?;
        let type_ = TurbineChatType::try_from(read_u32(reader)?)?;
        let blob_dispatch_type = read_u32(reader)?;
        let target_type = read_i32(reader)?;
        let target_id = read_i32(reader)?;
        let transport_type = read_i32(reader)?;
        let transport_id = read_i32(reader)?;
        let cookie = read_i32(reader)?;
        let payload_size = read_u32(reader)?;

        match type_ {
            TurbineChatType::ServerToClientMessage => {
                let variant_struct = CommunicationTurbineChatType1::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type1(variant_struct))
            },
            TurbineChatType::ClientToServerMessage => {
                let variant_struct = CommunicationTurbineChatType3::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type3(variant_struct))
            },
            TurbineChatType::AckClientToServerMessage => {
                let variant_struct = CommunicationTurbineChatType5::read(reader, mmessage_size, target_type, target_id, transport_type, transport_id, cookie, payload_size)?;
                Ok(Self::Type5(variant_struct))
            },
            _ => Err(format!("Unknown {} value: {:?}", "type_", type_).into()),
        }
    }
}

impl crate::readers::ACDataType for CommunicationTurbineChat {
    fn read(reader: &mut dyn ACReader) -> Result<Self, Box<dyn std::error::Error>> {
        CommunicationTurbineChat::read(reader)
    }
}

