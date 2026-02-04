use super::colors::*;
use miette::Result;
use netctl_core::NetworkManager;
use ratatui::{prelude::*, widgets::*};

pub struct App {
    pub manager: NetworkManager,
    pub should_quit: bool,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl App {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            manager: NetworkManager::new().await?,
            should_quit: false,
            selected_index: 0,
            scroll_offset: 0,
        })
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next(&mut self, max: usize) {
        if max > 0 {
            self.selected_index = (self.selected_index + 1).min(max - 1);
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub async fn render(&self, frame: &mut Frame<'_>) -> Result<()> {
        let area = frame.size();

        // Create main layout - split screen like guestkit
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Header
                Constraint::Min(0),      // Main content (split)
                Constraint::Length(3),   // Footer
            ])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0]);

        // Split main area into left and right panes
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),  // Interface list
                Constraint::Percentage(40),  // Details pane
            ])
            .split(chunks[1]);

        // Render interface list (left pane)
        self.render_interface_list(frame, main_chunks[0]).await?;

        // Render details pane (right pane)
        self.render_details(frame, main_chunks[1]).await?;

        // Render footer
        self.render_footer(frame, chunks[2]);

        Ok(())
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_text = vec![
            Line::from(vec![
                Span::styled("netctl", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::styled("Network Configuration Manager", Style::default().fg(LIGHT_ORANGE)),
            ])
        ];

        let header = Paragraph::new(header_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR))
                    .style(Style::default().bg(BG_COLOR)),
            );

        frame.render_widget(header, area);
    }

    async fn render_interface_list(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let interfaces = match self.manager.list_links().await {
            Ok(links) => links,
            Err(_) => Vec::new(),
        };

        if interfaces.is_empty() {
            let empty = Paragraph::new("⚠️  No network interfaces found")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .title(" 🌐 Network Interfaces ")
                        .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                )
                .style(Style::default().fg(TEXT_COLOR));
            frame.render_widget(empty, area);
            return Ok(());
        }

        let header = Row::new(vec!["", "Name", "State", "MTU", "MAC Address"])
            .style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD))
            .bottom_margin(1);

        let rows: Vec<Row> = interfaces
            .iter()
            .enumerate()
            .map(|(idx, iface)| {
                let state_color = match iface.state {
                    netctl_types::network::LinkState::Up => SUCCESS_COLOR,
                    netctl_types::network::LinkState::Down => ERROR_COLOR,
                };

                let mut style = Style::default().fg(state_color);
                if idx == self.selected_index {
                    style = style.bg(DARK_ORANGE).add_modifier(Modifier::BOLD);
                }

                let indicator = if idx == self.selected_index { "▶" } else { " " };

                Row::new(vec![
                    indicator.to_string(),
                    iface.name.clone(),
                    format!("{:?}", iface.state),
                    iface.mtu.to_string(),
                    iface.mac_address.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "-".to_string()),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(2),
                Constraint::Length(15),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(BORDER_COLOR))
                .title(format!(" 🌐 Network Interfaces ({}) ", interfaces.len()))
                .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
        )
        .column_spacing(2);

        frame.render_widget(table, area);
        Ok(())
    }

    async fn render_details(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let interfaces = match self.manager.list_links().await {
            Ok(links) => links,
            Err(_) => Vec::new(),
        };

        if let Some(iface) = interfaces.get(self.selected_index) {
            let mut lines = vec![
                Line::from(vec![
                    Span::styled("Interface: ", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                    Span::styled(&iface.name, Style::default().fg(LIGHT_ORANGE)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Index: ", Style::default().fg(TEXT_COLOR)),
                    Span::styled(iface.index.to_string(), Style::default().fg(LIGHT_ORANGE)),
                ]),
                Line::from(vec![
                    Span::styled("State: ", Style::default().fg(TEXT_COLOR)),
                    Span::styled(
                        format!("{:?}", iface.state),
                        Style::default().fg(match iface.state {
                            netctl_types::network::LinkState::Up => SUCCESS_COLOR,
                            netctl_types::network::LinkState::Down => ERROR_COLOR,
                        })
                    ),
                ]),
                Line::from(vec![
                    Span::styled("MTU: ", Style::default().fg(TEXT_COLOR)),
                    Span::styled(iface.mtu.to_string(), Style::default().fg(LIGHT_ORANGE)),
                ]),
            ];

            if let Some(ref mac) = iface.mac_address {
                lines.push(Line::from(vec![
                    Span::styled("MAC: ", Style::default().fg(TEXT_COLOR)),
                    Span::styled(mac.to_string(), Style::default().fg(LIGHT_ORANGE)),
                ]));
            }

            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("IP Addresses:", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            ]));

            if iface.addresses.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("  No addresses configured", Style::default().fg(TEXT_COLOR).add_modifier(Modifier::ITALIC)),
                ]));
            } else {
                for addr in &iface.addresses {
                    lines.push(Line::from(vec![
                        Span::raw("  • "),
                        Span::styled(addr.to_string(), Style::default().fg(SUCCESS_COLOR)),
                    ]));
                }
            }

            let details = Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .title(" 📋 Details ")
                        .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                )
                .wrap(Wrap { trim: true });

            frame.render_widget(details, area);
        } else {
            let empty = Paragraph::new("Select an interface to view details")
                .style(Style::default().fg(TEXT_COLOR).add_modifier(Modifier::ITALIC))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(BORDER_COLOR))
                        .title(" 📋 Details ")
                        .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                );
            frame.render_widget(empty, area);
        }

        Ok(())
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = vec![
            Line::from(vec![
                Span::styled("↑/↓", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" Navigate  ", Style::default().fg(TEXT_COLOR)),
                Span::styled("q/Esc", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" Quit  ", Style::default().fg(TEXT_COLOR)),
                Span::styled("Ctrl+C", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" Exit", Style::default().fg(TEXT_COLOR)),
            ])
        ];

        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR)),
            );

        frame.render_widget(footer, area);
    }
}
