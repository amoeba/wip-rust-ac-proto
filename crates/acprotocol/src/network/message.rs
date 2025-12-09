use super::reader::BinaryReader;
use std::io;

/// A parsed message extracted from assembled fragments
#[derive(Debug, Clone)]
pub struct ParsedMessage {
    /// Unique message ID
    pub id: u32,
    /// Opcode that identifies the message type
    pub opcode: u32,
    /// Raw message data (opcode + payload)
    pub data: Vec<u8>,
    /// Position in the message stream
    pub sequence: u32,
}

impl ParsedMessage {
    /// Parse a message from assembled fragment data
    pub fn from_fragment(data: Vec<u8>, sequence: u32, id: u32) -> io::Result<Self> {
        if data.len() < 4 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Message data too short to contain opcode",
            ));
        }

        let opcode = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        Ok(Self {
            id,
            opcode,
            data,
            sequence,
        })
    }

    /// Get a binary reader for reading message fields
    pub fn reader(&self) -> BinaryReader {
        BinaryReader::new(&self.data)
    }

    /// Get the opcode as hex string
    pub fn opcode_hex(&self) -> String {
        format!("0x{:04X}", self.opcode)
    }
}
