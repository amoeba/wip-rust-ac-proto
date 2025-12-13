use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod read {
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
}

mod tui {
    use anyhow::Result;
    use std::path::Path;

    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{
        backend::CrosstermBackend,
        layout::{Alignment, Constraint, Direction, Layout},
        prelude::Stylize,
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Paragraph, Row, Table},
        Terminal,
    };
    use serde_json::Value;
    use std::io;

    pub fn run(path: &Path) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Load pcap data
        let packets = load_packets(path)?;

        // Create app state
        let mut app = App::new(packets);

        // Run the TUI
        let res = run_app(&mut terminal, &mut app);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err);
        }

        Ok(())
    }

    struct PacketInfo {
        id: u32,
        direction: String,
        timestamp: String,
        flags: String,
        packet_type: String,
        size: usize,
        extra_info: String,
        opcode: u32,
        sequence: u32,
        queue: String,
        iteration: u32,
        port: u16,
        raw_json: String,
    }

    struct App {
        packets: Vec<PacketInfo>,
        selected: usize,
        scroll_offset: usize,
    }

    impl App {
        fn new(packets: Vec<PacketInfo>) -> Self {
            App {
                packets,
                selected: 0,
                scroll_offset: 0,
            }
        }

        fn next(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = (self.selected + 1) % self.packets.len();
                self.update_scroll(visible_rows);
            }
        }

        fn prev(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = if self.selected == 0 {
                    self.packets.len() - 1
                } else {
                    self.selected - 1
                };
                self.update_scroll(visible_rows);
            }
        }

        fn page_down(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = (self.selected + visible_rows).min(self.packets.len() - 1);
                self.update_scroll(visible_rows);
            }
        }

        fn page_up(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = self.selected.saturating_sub(visible_rows);
                self.update_scroll(visible_rows);
            }
        }

        fn update_scroll(&mut self, visible_rows: usize) {
            if visible_rows == 0 {
                return;
            }
            // Keep selected row visible
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            } else if self.selected >= self.scroll_offset + visible_rows {
                self.scroll_offset = self.selected.saturating_sub(visible_rows - 1);
            }
        }
    }

    fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
        loop {
            terminal.draw(|f| ui(f, app))?;

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    // Estimate visible rows (roughly 15-20 depending on screen)
                    let visible_rows = 15;
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => app.next(visible_rows),
                        KeyCode::Up | KeyCode::Char('k') => app.prev(visible_rows),
                        KeyCode::PageDown => app.page_down(visible_rows),
                        KeyCode::PageUp => app.page_up(visible_rows),
                        KeyCode::Home => {
                            app.selected = 0;
                            app.scroll_offset = 0;
                        }
                        KeyCode::End => {
                            if !app.packets.is_empty() {
                                app.selected = app.packets.len() - 1;
                                app.update_scroll(visible_rows);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui(f: &mut ratatui::Frame, app: &App) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(10),
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Pcap Packet Viewer")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Table
        let header = Row::new(vec![
            "#", "Dir", "Timestamp", "Flags", "Message Type", "Size", "OpCode", "Seq", "Queue", "Iter", "Port",
        ])
        .style(Style::default().bg(Color::DarkGray).bold())
        .bottom_margin(0);

        let visible_rows = chunks[1].height.saturating_sub(2) as usize; // Account for borders

        let rows: Vec<Row> = app
            .packets
            .iter()
            .enumerate()
            .skip(app.scroll_offset)
            .take(visible_rows)
            .map(|(i, packet)| {
                let cells = vec![
                    packet.id.to_string(),
                    packet.direction.clone(),
                    packet.timestamp.clone(),
                    packet.flags.clone(),
                    packet.packet_type.clone(),
                    packet.size.to_string(),
                    format!("0x{:04x}", packet.opcode),
                    packet.sequence.to_string(),
                    packet.queue.clone(),
                    packet.iteration.to_string(),
                    packet.port.to_string(),
                ];

                let style = if i == app.selected {
                    Style::default().bg(Color::Cyan).fg(Color::Black)
                } else {
                    Style::default()
                };

                Row::new(cells).style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(3),           // #
                Constraint::Length(5),           // Dir
                Constraint::Length(12),          // Timestamp
                Constraint::Length(16),          // Flags
                Constraint::Min(20),             // Message Type (expandable)
                Constraint::Length(6),           // Size
                Constraint::Length(8),           // OpCode
                Constraint::Length(6),           // Seq
                Constraint::Length(10),          // Queue
                Constraint::Length(4),           // Iter
                Constraint::Length(5),           // Port
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Packets"));

        f.render_widget(table, chunks[1]);

        // Details panel
        let detail_text = if !app.packets.is_empty() {
            let packet = &app.packets[app.selected];
            format!(
                "Packet #{} - {}\nMessage Type: {}\nRaw JSON:\n{}",
                packet.id,
                packet.direction,
                packet.packet_type,
                truncate_json(&packet.raw_json, 500)
            )
        } else {
            "No packets loaded".to_string()
        };

        let detail_para = Paragraph::new(detail_text)
            .block(Block::default().title("Details").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(detail_para, chunks[2]);
    }

    fn truncate_json(json: &str, max_len: usize) -> String {
        if json.len() > max_len {
            format!("{}...", &json[..max_len])
        } else {
            json.to_string()
        }
    }

    fn load_packets(path: &Path) -> Result<Vec<PacketInfo>> {
        use acprotocol::network::{FragmentAssembler, pcap};

        let mut pcap_iter = pcap::open(path.to_str().unwrap())?;
        let mut assembler = FragmentAssembler::new();
        let mut packets = Vec::new();
        let mut packet_num = 0u32;

        while let Some(packet_result) = pcap_iter.next() {
            let packet = packet_result?;

            // Extract port info from UDP header (before stripping headers)
            let dest_port = if packet.data.len() > 37 {
                u16::from_be_bytes([packet.data[36], packet.data[37]])
            } else {
                0
            };

            // Format timestamp
            let timestamp = format_timestamp(packet.ts_sec, packet.ts_usec);

            // Skip first 42 bytes (Ethernet + IP + UDP headers)
            if packet.data.len() <= 42 {
                continue;
            }
            let udp_payload = &packet.data[42..];

            // Try to parse messages from this packet
            match assembler.parse_packet_payload(udp_payload) {
                Ok(messages) => {
                    for msg in messages {
                        if let Ok(json_str) = serde_json::to_string(&msg) {
                            if let Ok(json_val) = serde_json::from_str::<Value>(&json_str) {
                                packet_num += 1;
                                let info = extract_packet_info(
                                    packet_num,
                                    &json_val,
                                    &json_str,
                                    &timestamp,
                                    dest_port,
                                );
                                packets.push(info);
                            }
                        }
                    }
                }
                Err(_) => {
                    // Silently skip packets that can't be parsed
                }
            }
        }

        Ok(packets)
    }

    fn format_timestamp(ts_sec: u32, ts_usec: u32) -> String {
        let millis = (ts_usec / 1000) % 1000;
        format!("{}.{:03}", ts_sec, millis)
    }

    fn format_packet_flags(flags_u32: u32) -> String {
        let mut flags = Vec::new();
        
        // Map of flag bits to names (from protocol.xml)
        const RETRANSMISSION: u32 = 0x00000001;
        const ENCRYPTED_CHECKSUM: u32 = 0x00000002;
        const BLOB_FRAGMENTS: u32 = 0x00000004;
        const SERVER_SWITCH: u32 = 0x00000100;
        const LOGON_SERVER_ADDR: u32 = 0x00000200;
        const EMPTY_HEADER1: u32 = 0x00000400;
        const REFERRAL: u32 = 0x00000800;
        const REQUEST_RETRANSMIT: u32 = 0x00001000;
        const REJECT_RETRANSMIT: u32 = 0x00002000;
        const ACK_SEQUENCE: u32 = 0x00004000;
        const DISCONNECT: u32 = 0x00008000;
        const LOGIN_REQUEST: u32 = 0x00010000;
        const WORLD_LOGIN_REQUEST: u32 = 0x00020000;
        const CONNECT_REQUEST: u32 = 0x00040000;
        const CONNECT_RESPONSE: u32 = 0x00080000;
        const NET_ERROR: u32 = 0x00100000;
        const NET_ERROR_DISCONNECT: u32 = 0x00200000;
        const CICMD_COMMAND: u32 = 0x00400000;
        const TIME_SYNC: u32 = 0x01000000;
        const ECHO_REQUEST: u32 = 0x02000000;
        const ECHO_RESPONSE: u32 = 0x04000000;
        const FLOW: u32 = 0x08000000;
        
        if flags_u32 & RETRANSMISSION != 0 {
            flags.push("Retrans");
        }
        if flags_u32 & ENCRYPTED_CHECKSUM != 0 {
            flags.push("EncCksum");
        }
        if flags_u32 & BLOB_FRAGMENTS != 0 {
            flags.push("BlobFrag");
        }
        if flags_u32 & SERVER_SWITCH != 0 {
            flags.push("SrvSwitch");
        }
        if flags_u32 & LOGON_SERVER_ADDR != 0 {
            flags.push("LogonAddr");
        }
        if flags_u32 & EMPTY_HEADER1 != 0 {
            flags.push("EmptyHdr1");
        }
        if flags_u32 & REFERRAL != 0 {
            flags.push("Referral");
        }
        if flags_u32 & REQUEST_RETRANSMIT != 0 {
            flags.push("ReqRetrans");
        }
        if flags_u32 & REJECT_RETRANSMIT != 0 {
            flags.push("RejRetrans");
        }
        if flags_u32 & ACK_SEQUENCE != 0 {
            flags.push("Ack");
        }
        if flags_u32 & DISCONNECT != 0 {
            flags.push("Disc");
        }
        if flags_u32 & LOGIN_REQUEST != 0 {
            flags.push("Login");
        }
        if flags_u32 & WORLD_LOGIN_REQUEST != 0 {
            flags.push("WorldLogin");
        }
        if flags_u32 & CONNECT_REQUEST != 0 {
            flags.push("ConnectReq");
        }
        if flags_u32 & CONNECT_RESPONSE != 0 {
            flags.push("ConnectResp");
        }
        if flags_u32 & NET_ERROR != 0 {
            flags.push("NetErr");
        }
        if flags_u32 & NET_ERROR_DISCONNECT != 0 {
            flags.push("NetErrDisc");
        }
        if flags_u32 & CICMD_COMMAND != 0 {
            flags.push("CICMD");
        }
        if flags_u32 & TIME_SYNC != 0 {
            flags.push("TimeSync");
        }
        if flags_u32 & ECHO_REQUEST != 0 {
            flags.push("EchoReq");
        }
        if flags_u32 & ECHO_RESPONSE != 0 {
            flags.push("EchoResp");
        }
        if flags_u32 & FLOW != 0 {
            flags.push("Flow");
        }
        
        if flags.is_empty() {
            "None".to_string()
        } else {
            flags.join("|")
        }
    }

    fn extract_packet_info(
        id: u32,
        json_val: &Value,
        raw_json: &str,
        timestamp: &str,
        port: u16,
    ) -> PacketInfo {
        let id_from_json = json_val.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let direction = json_val
            .get("direction")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        let opcode = json_val
            .get("opcode")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let sequence = json_val
            .get("sequence")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let message_type = json_val
            .get("message_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        let iteration = json_val
            .get("iteration")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;

        // Calculate approximate size (raw JSON string length)
        let size = raw_json.len();

        // Extract flags from header_flags field if present
        let flags = json_val
            .get("header_flags")
            .and_then(|v| v.as_u64())
            .map(|f| format_packet_flags(f as u32))
            .unwrap_or_else(|| "Unknown".to_string());

        // Extract queue from JSON (already populated from codegen)
        let queue = json_val
            .get("queue")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        PacketInfo {
            id,
            direction,
            timestamp: timestamp.to_string(),
            flags,
            packet_type: message_type,
            size,
            extra_info: format!("ID:{}", id_from_json),
            opcode,
            sequence,
            queue,
            iteration,
            port,
            raw_json: raw_json.to_string(),
        }
    }
}

#[derive(Parser)]
#[command(name = "pcap")]
#[command(about = "Parse .pcap network capture files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read and print packet data from a pcap file
    Read {
        /// Path to .pcap file
        path: PathBuf,
    },
    /// Interactive view for browsing pcap files
    View {
        /// Path to .pcap file
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { path } => read::run(&path)?,
        Commands::View { path } => tui::run(&path)?,
    }

    Ok(())
}
