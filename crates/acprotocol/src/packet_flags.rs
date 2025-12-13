use crate::enums::PacketHeaderFlags;

/// Mapping of flag names to their short display forms for the TUI
pub fn format_packet_flags(flags: PacketHeaderFlags) -> String {
    let mut flag_names = Vec::new();

    if flags.contains(PacketHeaderFlags::RETRANSMISSION) {
        flag_names.push("Retrans");
    }
    if flags.contains(PacketHeaderFlags::ENCRYPTED_CHECKSUM) {
        flag_names.push("EncCksum");
    }
    if flags.contains(PacketHeaderFlags::BLOB_FRAGMENTS) {
        flag_names.push("BlobFrag");
    }
    if flags.contains(PacketHeaderFlags::SERVER_SWITCH) {
        flag_names.push("SrvSwitch");
    }
    if flags.contains(PacketHeaderFlags::LOGON_SERVER_ADDR) {
        flag_names.push("LogonAddr");
    }
    if flags.contains(PacketHeaderFlags::EMPTY_HEADER1) {
        flag_names.push("EmptyHdr1");
    }
    if flags.contains(PacketHeaderFlags::REFERRAL) {
        flag_names.push("Referral");
    }
    if flags.contains(PacketHeaderFlags::REQUEST_RETRANSMIT) {
        flag_names.push("ReqRetrans");
    }
    if flags.contains(PacketHeaderFlags::REJECT_RETRANSMIT) {
        flag_names.push("RejRetrans");
    }
    if flags.contains(PacketHeaderFlags::ACK_SEQUENCE) {
        flag_names.push("Ack");
    }
    if flags.contains(PacketHeaderFlags::DISCONNECT) {
        flag_names.push("Disc");
    }
    if flags.contains(PacketHeaderFlags::LOGIN_REQUEST) {
        flag_names.push("Login");
    }
    if flags.contains(PacketHeaderFlags::WORLD_LOGIN_REQUEST) {
        flag_names.push("WorldLogin");
    }
    if flags.contains(PacketHeaderFlags::CONNECT_REQUEST) {
        flag_names.push("ConnectReq");
    }
    if flags.contains(PacketHeaderFlags::CONNECT_RESPONSE) {
        flag_names.push("ConnectResp");
    }
    if flags.contains(PacketHeaderFlags::NET_ERROR) {
        flag_names.push("NetErr");
    }
    if flags.contains(PacketHeaderFlags::NET_ERROR_DISCONNECT) {
        flag_names.push("NetErrDisc");
    }
    if flags.contains(PacketHeaderFlags::CICMDCOMMAND) {
        flag_names.push("CICMD");
    }
    if flags.contains(PacketHeaderFlags::TIME_SYNC) {
        flag_names.push("TimeSync");
    }
    if flags.contains(PacketHeaderFlags::ECHO_REQUEST) {
        flag_names.push("EchoReq");
    }
    if flags.contains(PacketHeaderFlags::ECHO_RESPONSE) {
        flag_names.push("EchoResp");
    }
    if flags.contains(PacketHeaderFlags::FLOW) {
        flag_names.push("Flow");
    }

    if flag_names.is_empty() {
        "None".to_string()
    } else {
        flag_names.join("|")
    }
}
