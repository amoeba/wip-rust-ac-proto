use std::collections::HashMap;
use std::io;

use crate::generated::network::{Fragment, FragmentHeader};

use super::message::ParsedMessage;
use super::packet::{PacketHeader, PacketHeaderFlags};
use super::reader::BinaryReader;

/// Information about a fragment extracted from a packet
#[derive(Debug, Clone)]
pub struct ExtractedFragment {
    pub sequence: u32,
    pub id: u32,
    pub index: u16,
    pub count: u16,
    pub data: Vec<u8>,
    pub is_complete: bool,
}

/// Parses packets and assembles fragments into complete messages
pub struct FragmentAssembler {
    pending_fragments: HashMap<u32, Fragment>,
    next_message_id: u32,
}

impl FragmentAssembler {
    pub fn new() -> Self {
        Self {
            pending_fragments: HashMap::new(),
            next_message_id: 0,
        }
    }

    /// Parse a network packet's payload and extract fragments, returning any completed messages
    pub fn parse_packet_payload(&mut self, payload: &[u8]) -> io::Result<Vec<ParsedMessage>> {
        let mut completed_messages = Vec::new();
        let mut reader = BinaryReader::new(payload);

        while reader.remaining() > 0 {
            let start_pos = reader.position();

            // Parse packet header
            let header = PacketHeader::parse(&mut reader)?;

            // Parse optional headers based on flags
            if header.flags.contains(PacketHeaderFlags::REQUEST_RETRANSMIT) {
                let num = reader.read_u32()?;
                for _ in 0..num {
                    reader.read_u32()?; // sequence id
                }
            }

            if header.flags.contains(PacketHeaderFlags::ACK_SEQUENCE) {
                reader.read_u32()?; // sequence
            }

            // Calculate packet boundaries
            let packet_end = start_pos + PacketHeader::BASE_SIZE + header.size as usize;

            // If this packet has fragments, parse them
            if header.flags.contains(PacketHeaderFlags::BLOB_FRAGMENTS) {
                while reader.position() < packet_end && reader.remaining() > 0 {
                    match self.parse_fragment(&mut reader) {
                        Ok(Some(msg)) => {
                            completed_messages.push(msg);
                        }
                        Ok(None) => {
                            // Fragment received but not complete yet
                        }
                        Err(_) => break,
                    }
                }
            }

            // Move to next packet
            if reader.position() < packet_end {
                reader.set_position(packet_end);
            }
        }

        Ok(completed_messages)
    }

    /// Parse a single fragment from the reader
    /// Returns Some(ParsedMessage) if the fragment completes a message, None otherwise
    fn parse_fragment(&mut self, reader: &mut BinaryReader) -> io::Result<Option<ParsedMessage>> {
        let sequence = reader.read_u32()?;
        let id = reader.read_u32()?;
        let count = reader.read_u16()?;
        let size = reader.read_u16()?;
        let index = reader.read_u16()?;
        let group = reader.read_u16()?;

        if size < 16 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid fragment size",
            ));
        }

        let frag_length = size as usize - 16;

        if reader.remaining() < frag_length {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Fragment data too short",
            ));
        }

        let data = reader.read_bytes(frag_length)?;

        // Update or create fragment entry
        let fragment = self
            .pending_fragments
            .entry(sequence)
            .or_insert_with(|| Fragment::new(sequence, count));

        fragment.add_chunk(&data, index as usize);
        fragment.header = FragmentHeader {
            sequence,
            id,
            count,
            size,
            index,
            group,
        };

        // Check if this completes the fragment assembly
        if fragment.is_complete() {
            let assembled_data = fragment.get_data().to_vec();
            self.pending_fragments.remove(&sequence);

            // Try to parse as a message
            let msg_id = self.next_message_id;
            self.next_message_id += 1;

            let parsed_msg = ParsedMessage::from_fragment(assembled_data, sequence, msg_id)?;
            Ok(Some(parsed_msg))
        } else {
            Ok(None)
        }
    }
}

impl Default for FragmentAssembler {
    fn default() -> Self {
        Self::new()
    }
}
