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

    #[derive(Clone)]
    struct TreeNode {
        key: String,
        value: String,
        children: Vec<TreeNode>,
        expanded: bool,
    }

    impl TreeNode {
        fn from_json(key: &str, value: &Value) -> Self {
            let (value_str, children) = match value {
                Value::Object(obj) => {
                    let val = format!("{{...}}", );
                    let children: Vec<TreeNode> = obj
                        .iter()
                        .map(|(k, v)| TreeNode::from_json(k, v))
                        .collect();
                    (val, children)
                }
                Value::Array(arr) => {
                    let val = format!("[{}]", arr.len());
                    let children: Vec<TreeNode> = arr
                        .iter()
                        .enumerate()
                        .map(|(i, v)| TreeNode::from_json(&format!("[{}]", i), v))
                        .collect();
                    (val, children)
                }
                _ => (value.to_string(), Vec::new()),
            };

            TreeNode {
                key: key.to_string(),
                value: value_str,
                children,
                expanded: false,
            }
        }

        fn get_display_lines(&self, depth: usize, expanded_set: &std::collections::HashSet<String>, path: String) -> Vec<(String, String)> {
            let mut lines = Vec::new();
            let current_path = if path.is_empty() {
                self.key.clone()
            } else {
                format!("{}.{}", path, self.key)
            };

            let prefix = "  ".repeat(depth);
            let expanded = expanded_set.contains(&current_path);
            let toggle = if !self.children.is_empty() {
                if expanded { "▼" } else { "▶" }
            } else {
                " "
            };

            lines.push((
                format!("{}{} {}: {}", prefix, toggle, self.key, self.value),
                current_path.clone(),
            ));

            if expanded && !self.children.is_empty() {
                for child in &self.children {
                    lines.extend(child.get_display_lines(depth + 1, expanded_set, current_path.clone()));
                }
            }

            lines
        }
    }

    struct App {
        packets: Vec<PacketInfo>,
        selected: usize,
        scroll_offset: usize,
        tree_expanded: std::collections::HashSet<String>,
        tree_scroll_offset: usize,
        tree_focused_line: usize,
        detail_mode: DetailMode,
        focused_pane: FocusedPane,
    }

    enum DetailMode {
        JSON,
        Tree,
    }

    enum FocusedPane {
        List,
        Details,
    }

    impl App {
        fn new(packets: Vec<PacketInfo>) -> Self {
            App {
                packets,
                selected: 0,
                scroll_offset: 0,
                tree_expanded: std::collections::HashSet::new(),
                tree_scroll_offset: 0,
                tree_focused_line: 0,
                detail_mode: DetailMode::Tree,
                focused_pane: FocusedPane::List,
            }
        }

        fn next(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = (self.selected + 1) % self.packets.len();
                self.update_scroll(visible_rows);
                self.tree_scroll_offset = 0; // Reset tree scroll when changing packet
                self.tree_focused_line = 0;
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
                self.tree_scroll_offset = 0; // Reset tree scroll when changing packet
                self.tree_focused_line = 0;
            }
        }

        fn page_down(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = (self.selected + visible_rows).min(self.packets.len() - 1);
                self.update_scroll(visible_rows);
                self.tree_scroll_offset = 0; // Reset tree scroll when changing packet
                self.tree_focused_line = 0;
            }
        }

        fn page_up(&mut self, visible_rows: usize) {
            if !self.packets.is_empty() {
                self.selected = self.selected.saturating_sub(visible_rows);
                self.update_scroll(visible_rows);
                self.tree_scroll_offset = 0; // Reset tree scroll when changing packet
                self.tree_focused_line = 0;
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

        fn toggle_tree_node(&mut self, path: String) {
            if self.tree_expanded.contains(&path) {
                self.tree_expanded.remove(&path);
            } else {
                self.tree_expanded.insert(path);
            }
        }

        fn tree_scroll_up(&mut self) {
            self.tree_scroll_offset = self.tree_scroll_offset.saturating_sub(1);
        }

        fn tree_scroll_down(&mut self) {
            self.tree_scroll_offset = self.tree_scroll_offset.saturating_add(1);
        }

        fn tree_line_down(&mut self, total_lines: usize, visible_rows: usize) {
            if self.tree_focused_line < total_lines.saturating_sub(1) {
                self.tree_focused_line += 1;
                // Auto-scroll to keep focused line visible
                if self.tree_focused_line >= self.tree_scroll_offset + visible_rows {
                    self.tree_scroll_offset = self.tree_focused_line.saturating_sub(visible_rows - 1);
                }
            }
        }

        fn tree_line_up(&mut self) {
            if self.tree_focused_line > 0 {
                self.tree_focused_line -= 1;
                // Auto-scroll to keep focused line visible
                if self.tree_focused_line < self.tree_scroll_offset {
                    self.tree_scroll_offset = self.tree_focused_line;
                }
            }
        }
    }

    fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
        loop {
            // Get detail lines before drawing (for Enter key handling)
            let detail_lines = if !app.packets.is_empty() {
                let packet = &app.packets[app.selected];
                match serde_json::from_str::<Value>(&packet.raw_json) {
                    Ok(json_val) => {
                        let mut expanded = app.tree_expanded.clone();
                        expanded.insert("root".to_string()); // Root is always expanded
                        let root = TreeNode::from_json("root", &json_val);
                        root.get_display_lines(0, &expanded, String::new())
                    }
                    Err(_) => vec![],
                }
            } else {
                vec![]
            };

            terminal.draw(|f| ui(f, app))?;

            if crossterm::event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    // Estimate visible rows (roughly 15-20 depending on screen)
                    let visible_rows = 15;
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                        KeyCode::Tab => {
                            // Switch between panes
                            app.focused_pane = match app.focused_pane {
                                FocusedPane::List => FocusedPane::Details,
                                FocusedPane::Details => FocusedPane::List,
                            };
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            match app.focused_pane {
                                FocusedPane::List => app.next(visible_rows),
                                FocusedPane::Details => {
                                    let total_lines = detail_lines.len();
                                    app.tree_line_down(total_lines, visible_rows);
                                }
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            match app.focused_pane {
                                FocusedPane::List => app.prev(visible_rows),
                                FocusedPane::Details => app.tree_line_up(),
                            }
                        }
                        KeyCode::PageDown => app.page_down(visible_rows),
                        KeyCode::PageUp => app.page_up(visible_rows),
                        KeyCode::Home => {
                            app.selected = 0;
                            app.scroll_offset = 0;
                            app.tree_scroll_offset = 0;
                            app.tree_focused_line = 0;
                        }
                        KeyCode::End => {
                            if !app.packets.is_empty() {
                                app.selected = app.packets.len() - 1;
                                app.update_scroll(visible_rows);
                            }
                        }
                        KeyCode::Enter => {
                            // Only expand/collapse in details pane
                            if matches!(app.focused_pane, FocusedPane::Details) {
                                let focused_global_line = app.tree_scroll_offset + app.tree_focused_line;
                                if focused_global_line < detail_lines.len() {
                                    let (line, path) = &detail_lines[focused_global_line];
                                    // Only toggle if line contains expand/collapse markers
                                    if (line.contains("▶") || line.contains("▼")) && !path.is_empty() {
                                        app.toggle_tree_node(path.clone());
                                    }
                                }
                            }
                        }
                        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Ctrl+f: page down
                            match app.focused_pane {
                                FocusedPane::List => app.page_down(visible_rows),
                                FocusedPane::Details => {
                                    for _ in 0..visible_rows {
                                        let total_lines = detail_lines.len();
                                        app.tree_line_down(total_lines, visible_rows);
                                    }
                                }
                            }
                        }
                        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            // Ctrl+b: page up
                            match app.focused_pane {
                                FocusedPane::List => app.page_up(visible_rows),
                                FocusedPane::Details => {
                                    for _ in 0..visible_rows {
                                        app.tree_line_up();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui(f: &mut ratatui::Frame, app: &App) {
        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(f.area());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(66),
                Constraint::Percentage(34),
            ])
            .split(outer_chunks[0]);

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
                } else if !matches!(app.focused_pane, FocusedPane::List) {
                    Style::default().add_modifier(Modifier::DIM)
                } else {
                    Style::default()
                };

                Row::new(cells).style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(2),           // #
                Constraint::Length(4),           // Dir
                Constraint::Length(8),           // Timestamp
                Constraint::Length(8),           // Flags
                Constraint::Min(15),             // Message Type (expandable)
                Constraint::Length(5),           // Size
                Constraint::Length(6),           // OpCode
                Constraint::Length(4),           // Seq
                Constraint::Length(7),           // Queue
                Constraint::Length(3),           // Iter
                Constraint::Length(4),           // Port
            ],
        )
        .header(header)
        .block({
            let style = if matches!(app.focused_pane, FocusedPane::List) {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Block::default()
                .borders(Borders::ALL)
                .title("Packets")
                .style(style)
        });

        f.render_widget(table, chunks[0]);

        // Details panel
        let detail_title = "Details";

        if !app.packets.is_empty() {
            let packet = &app.packets[app.selected];
            
            // Parse JSON and build tree
            let detail_lines = match serde_json::from_str::<Value>(&packet.raw_json) {
                Ok(json_val) => {
                    let mut expanded = app.tree_expanded.clone();
                    expanded.insert("root".to_string()); // Root is always expanded
                    let root = TreeNode::from_json("root", &json_val);
                    root.get_display_lines(0, &expanded, String::new())
                }
                Err(e) => {
                    vec![(format!("Failed to parse JSON: {}\n\nRaw:\n{}", e, &packet.raw_json), String::new())]
                }
            };
            
            let visible_rows = chunks[1].height.saturating_sub(2) as usize;
            let mut detail_text = String::new();
            
            // Render visible lines with focused line highlighted
            for (i, (line, _path)) in detail_lines.iter().enumerate() {
                if i >= app.tree_scroll_offset && i < app.tree_scroll_offset + visible_rows {
                    let display_idx = i - app.tree_scroll_offset;
                    if display_idx == app.tree_focused_line {
                        detail_text.push_str(&format!("► {}\n", line));
                    } else {
                        detail_text.push_str(&format!("{}\n", line));
                    }
                }
            }
            
            let text_style = if matches!(app.focused_pane, FocusedPane::Details) {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            };
            
            let detail_para = Paragraph::new(detail_text)
                .block({
                    let style = if matches!(app.focused_pane, FocusedPane::Details) {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    Block::default()
                        .title(detail_title)
                        .borders(Borders::ALL)
                        .style(style)
                })
                .style(text_style);
            f.render_widget(detail_para, chunks[1]);
        } else {
            let text_style = if matches!(app.focused_pane, FocusedPane::Details) {
                Style::default()
            } else {
                Style::default().add_modifier(Modifier::DIM)
            };
            
            let detail_para = Paragraph::new("No packets loaded")
                .block({
                    let style = if matches!(app.focused_pane, FocusedPane::Details) {
                        Style::default().add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    Block::default()
                        .title(detail_title)
                        .borders(Borders::ALL)
                        .style(style)
                })
                .style(text_style);
            f.render_widget(detail_para, chunks[1]);
        }

        // Footer with controls
        let controls_text = "q/Esc: Quit | Tab: Switch pane | ↑/↓: Navigate | Enter: Expand/collapse";
        let controls = Paragraph::new(controls_text)
            .block(Block::default().borders(Borders::TOP));
        f.render_widget(controls, outer_chunks[1]);
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
