use super::message_parser::parse_message_to_json;
use super::reader::BinaryReader;
use serde::Serialize;
use std::io;

/// A parsed message extracted from assembled fragments
#[derive(Debug, Clone, Serialize)]
pub struct ParsedMessage {
    /// Unique message ID
    pub id: u32,
    /// Opcode that identifies the message type (as number)
    pub opcode: u32,
    /// Human-readable message type name
    pub message_type: String,
    /// Message direction (Send/Recv)
    pub direction: String,
    /// Parsed message data as JSON, or raw hex if parsing fails
    #[serde(serialize_with = "serialize_parsed_data")]
    pub data: Vec<u8>,
    /// Position in the message stream
    pub sequence: u32,
}

fn serialize_parsed_data<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Extract opcode from the data
    if data.len() < 4 {
        return serializer.serialize_str("invalid");
    }

    let opcode = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

    // Try to parse the data as a structured message
    // STRICT MODE: Fail loudly on any parsing error instead of falling back to hex
    match parse_message_to_json(opcode, data) {
        Ok(json_value) => json_value.serialize(serializer),
        Err(e) => {
            // STRICT: Always fail and panic with full error details
            let hex_string: String = data.iter().map(|b| format!("{:02x}", b)).collect();
            panic!(
                "STRICT PARSING FAILED for opcode 0x{:04x}:\n  Error: {}\n  Raw data (hex): {}",
                opcode, e, hex_string
            );
        }
    }
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

        // Create a temporary message to get type name and direction
        let message = Self {
            id,
            opcode,
            message_type: String::new(),
            direction: String::new(),
            data,
            sequence,
        };

        let message_type = message.message_type_name();
        let direction = message.direction().to_string();

        Ok(Self {
            id: message.id,
            opcode: message.opcode,
            message_type,
            direction,
            data: message.data,
            sequence: message.sequence,
        })
    }

    /// Get a binary reader for reading message fields
    pub fn reader(&self) -> BinaryReader<'_> {
        BinaryReader::new(&self.data)
    }

    /// Get the opcode as hex string
    pub fn opcode_hex(&self) -> String {
        format!("0x{:04X}", self.opcode)
    }

    /// Get the human-readable message type name
    pub fn message_type_name(&self) -> String {
        use crate::generated::enums::{C2SMessage, GameAction, GameEvent, S2CMessage};

        // Skip the first 4 bytes which is the opcode, and then try to identify the inner message type
        // for ordered messages
        if self.data.len() < 4 {
            return "Unknown".to_string();
        }

        let payload = &self.data[4..]; // Skip the outer opcode

        // Based on the outer opcode, interpret the inner payload
        if let Ok(msg_type) = C2SMessage::try_from(self.opcode) {
            if msg_type == C2SMessage::OrderedGameAction && payload.len() >= 8 {
                // For OrderedGameAction: [sequence (4)] [action_type (4)] [payload...]
                let action_type_val =
                    u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]);
                if let Ok(game_action) = GameAction::try_from(action_type_val) {
                    return format!("{:?}", game_action);
                }
                // Fallback to the outer type if we can't parse the inner type
                return "OrderedGameAction".to_string();
            }
            return format!("{:?}", msg_type);
        }

        if let Ok(msg_type) = S2CMessage::try_from(self.opcode) {
            if msg_type == S2CMessage::OrderedGameEvent && payload.len() >= 12 {
                // For OrderedGameEvent: [object_id (4)] [sequence (4)] [event_type (4)] [payload...]
                let event_type_val =
                    u32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]);
                if let Ok(game_event) = GameEvent::try_from(event_type_val) {
                    return format!("{:?}", game_event);
                }
                // Fallback to the outer type if we can't parse the inner type
                return "OrderedGameEvent".to_string();
            }
            return format!("{:?}", msg_type);
        }

        "Unknown".to_string()
    }

    /// Get the message direction (Send/Recv)
    pub fn direction(&self) -> &'static str {
        use crate::generated::enums::{C2SMessage, S2CMessage};

        if C2SMessage::try_from(self.opcode).is_ok() {
            "Send"
        } else if S2CMessage::try_from(self.opcode).is_ok() {
            "Recv"
        } else {
            "Unknown"
        }
    }
}
