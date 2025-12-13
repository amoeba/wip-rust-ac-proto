use anyhow::Result;
use std::path::Path;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
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

struct App {
    packets: Vec<String>,
    selected: usize,
}

impl App {
    fn new(packets: Vec<String>) -> Self {
        App {
            packets,
            selected: 0,
        }
    }

    fn next(&mut self) {
        if !self.packets.is_empty() {
            self.selected = (self.selected + 1) % self.packets.len();
        }
    }

    fn prev(&mut self) {
        if !self.packets.is_empty() {
            self.selected = if self.selected == 0 {
                self.packets.len() - 1
            } else {
                self.selected - 1
            };
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.prev(),
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
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Pcap Viewer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Packet list and details
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    // Packet list
    let mut list_text = Vec::new();
    for (i, _) in app.packets.iter().enumerate() {
        let style = if i == app.selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
        } else {
            Style::default()
        };
        list_text.push(Line::from(Span::styled(
            format!("Packet {}", i),
            style,
        )));
    }

    let packet_list = Paragraph::new(list_text)
        .block(Block::default().title("Packets").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(packet_list, content_chunks[0]);

    // Packet details
    let detail_text = if !app.packets.is_empty() {
        app.packets[app.selected].clone()
    } else {
        "No packets loaded".to_string()
    };

    let packet_detail = Paragraph::new(detail_text)
        .block(Block::default().title("Details").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(packet_detail, content_chunks[1]);

    // Footer
    let footer = Paragraph::new("↑/k: Up | ↓/j: Down | q/Esc: Quit")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn load_packets(path: &Path) -> Result<Vec<String>> {
    use acprotocol::network::{FragmentAssembler, pcap};

    let mut pcap_iter = pcap::open(path.to_str().unwrap())?;
    let mut assembler = FragmentAssembler::new();
    let mut packets = Vec::new();

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
                    if let Ok(json_str) = serde_json::to_string_pretty(&msg) {
                        packets.push(json_str);
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
