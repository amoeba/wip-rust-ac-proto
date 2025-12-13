use anyhow::Result;
use std::path::Path;

pub fn run(path: &Path) -> Result<()> {
    use acprotocol::network::{FragmentAssembler, pcap};

    let mut pcap_iter = pcap::open(path.to_str().unwrap())?;
    let mut assembler = FragmentAssembler::new();

    while let Some(packet_result) = pcap_iter.next() {
        let packet = packet_result?;

        // Skip first 42 bytes (Ethernet + IP + UDP headers)
        if packet.data.len() <= 42 {
            continue;
        }
        let udp_payload = &packet.data[42..];

        // Try to parse messages from this packet
        match assembler.parse_packet_payload(udp_payload) {
            Ok(messages) => {
                for msg in messages {
                    println!("{}", serde_json::to_string(&msg)?);
                }
            }
            Err(_) => {
                // Silently skip packets that can't be parsed
            }
        }
    }

    Ok(())
}
