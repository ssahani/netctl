use super::colors::*;
use miette::Result;
use netctl_core::NetworkManager;
use ratatui::{prelude::*, widgets::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Statistics,
    Configuration,
}

impl View {
    pub fn title(&self) -> &str {
        match self {
            View::Dashboard => "Dashboard",
            View::Statistics => "Statistics",
            View::Configuration => "Configuration",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            View::Dashboard => "📊",
            View::Statistics => "📈",
            View::Configuration => "⚙️",
        }
    }

    pub fn all() -> Vec<View> {
        vec![View::Dashboard, View::Statistics, View::Configuration]
    }
}

pub struct App {
    pub manager: NetworkManager,
    pub should_quit: bool,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub current_view: View,
    pub show_help: bool,
    pub show_stats_bar: bool,
    pub search_query: String,
    pub is_searching: bool,
}

impl App {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            manager: NetworkManager::new().await?,
            should_quit: false,
            selected_index: 0,
            scroll_offset: 0,
            current_view: View::Dashboard,
            show_help: false,
            show_stats_bar: true,
            search_query: String::new(),
            is_searching: false,
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

    pub fn next_view(&mut self) {
        let views = View::all();
        let current_idx = views.iter().position(|v| v == &self.current_view).unwrap_or(0);
        self.current_view = views[(current_idx + 1) % views.len()];
        self.selected_index = 0;
    }

    pub fn previous_view(&mut self) {
        let views = View::all();
        let current_idx = views.iter().position(|v| v == &self.current_view).unwrap_or(0);
        self.current_view = views[(current_idx + views.len() - 1) % views.len()];
        self.selected_index = 0;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_stats_bar(&mut self) {
        self.show_stats_bar = !self.show_stats_bar;
    }

    pub fn start_search(&mut self) {
        self.is_searching = true;
        self.search_query.clear();
    }

    pub fn cancel_search(&mut self) {
        self.is_searching = false;
        self.search_query.clear();
    }

    pub fn append_to_search(&mut self, c: char) {
        self.search_query.push(c);
    }

    pub fn backspace_search(&mut self) {
        self.search_query.pop();
    }

    pub async fn render(&self, frame: &mut Frame<'_>) -> Result<()> {
        let area = frame.size();

        // Create main layout with optional stats bar
        let constraints = if self.show_stats_bar {
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(2),  // Stats bar
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ]
        } else {
            vec![
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Tabs
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ]
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        let mut idx = 0;

        // Render header
        self.render_header(frame, chunks[idx]);
        idx += 1;

        // Render stats bar if enabled
        if self.show_stats_bar {
            self.render_stats_bar(frame, chunks[idx]).await?;
            idx += 1;
        }

        // Render tabs
        self.render_tabs(frame, chunks[idx]);
        idx += 1;

        // Render current view
        match self.current_view {
            View::Dashboard => self.render_dashboard(frame, chunks[idx]).await?,
            View::Statistics => self.render_statistics(frame, chunks[idx]).await?,
            View::Configuration => self.render_configuration(frame, chunks[idx]).await?,
        }
        idx += 1;

        // Render footer
        self.render_footer(frame, chunks[idx]);

        // Render help overlay if active
        if self.show_help {
            self.render_help_overlay(frame);
        }

        Ok(())
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_text = vec![
            Line::from(vec![
                Span::styled("netctl", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::styled("Network Configuration Manager", Style::default().fg(LIGHT_ORANGE)),
                Span::raw("  │  "),
                Span::raw(format!("{} ", self.current_view.icon())),
                Span::styled(self.current_view.title(), Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
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

    async fn render_stats_bar(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let interfaces = match self.manager.list_links().await {
            Ok(links) => links,
            Err(_) => Vec::new(),
        };

        let total = interfaces.len();
        let up = interfaces.iter().filter(|i| matches!(i.state, netctl_types::network::LinkState::Up)).count();
        let down = total - up;

        let stats_text = Line::from(vec![
            Span::styled("📊 ", Style::default().fg(ORANGE)),
            Span::styled("Total:", Style::default().fg(LIGHT_ORANGE)),
            Span::styled(format!(" {} ", total), Style::default().fg(TEXT_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw("│ "),
            Span::styled("Up:", Style::default().fg(LIGHT_ORANGE)),
            Span::styled(format!(" {} ", up), Style::default().fg(SUCCESS_COLOR).add_modifier(Modifier::BOLD)),
            Span::raw("│ "),
            Span::styled("Down:", Style::default().fg(LIGHT_ORANGE)),
            Span::styled(format!(" {} ", down), Style::default().fg(ERROR_COLOR).add_modifier(Modifier::BOLD)),
        ]);

        let stats = Paragraph::new(stats_text)
            .alignment(Alignment::Center)
            .style(Style::default().bg(BG_COLOR));

        frame.render_widget(stats, area);
        Ok(())
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let views = View::all();
        let titles: Vec<Line> = views
            .iter()
            .map(|view| {
                let style = if view == &self.current_view {
                    Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(TEXT_COLOR)
                };
                Line::from(vec![
                    Span::raw(format!("{} ", view.icon())),
                    Span::styled(view.title(), style),
                ])
            })
            .collect();

        let selected_idx = views.iter().position(|v| v == &self.current_view).unwrap_or(0);

        let tabs = Tabs::new(titles)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR)),
            )
            .select(selected_idx)
            .highlight_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
    }

    async fn render_dashboard(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        // Split main area into left and right panes
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(60),  // Interface list
                Constraint::Percentage(40),  // Details pane
            ])
            .split(area);

        // Render interface list (left pane)
        self.render_interface_list(frame, main_chunks[0]).await?;

        // Render details pane (right pane)
        self.render_details(frame, main_chunks[1]).await?;

        Ok(())
    }

    async fn render_statistics(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let interfaces = match self.manager.list_links().await {
            Ok(links) => links,
            Err(_) => Vec::new(),
        };

        let total = interfaces.len() as u16;
        let up = interfaces.iter().filter(|i| matches!(i.state, netctl_types::network::LinkState::Up)).count() as u16;
        let up_pct = if total > 0 { (up * 100) / total } else { 0 };

        // Split into gauges
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),  // Interface gauge
                Constraint::Min(0),     // Statistics table
            ])
            .split(area);

        // Interface status gauge
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR))
                    .title(" 🌐 Interface Status ")
                    .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            )
            .gauge_style(Style::default().fg(SUCCESS_COLOR))
            .percent(up_pct)
            .label(format!("{}/{} interfaces up ({}%)", up, total, up_pct));

        frame.render_widget(gauge, chunks[0]);

        // Statistics info
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Network Statistics", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Total Interfaces: ", Style::default().fg(TEXT_COLOR)),
                Span::styled(total.to_string(), Style::default().fg(LIGHT_ORANGE)),
            ]),
            Line::from(vec![
                Span::styled("Active Interfaces: ", Style::default().fg(TEXT_COLOR)),
                Span::styled(up.to_string(), Style::default().fg(SUCCESS_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("Inactive Interfaces: ", Style::default().fg(TEXT_COLOR)),
                Span::styled((total - up).to_string(), Style::default().fg(ERROR_COLOR)),
            ]),
        ];

        for iface in &interfaces {
            let state_text = match iface.state {
                netctl_types::network::LinkState::Up => Span::styled("UP", Style::default().fg(SUCCESS_COLOR)),
                netctl_types::network::LinkState::Down => Span::styled("DOWN", Style::default().fg(ERROR_COLOR)),
            };

            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(&iface.name, Style::default().fg(LIGHT_ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(": "),
                state_text,
                Span::raw(format!(" (MTU: {})", iface.mtu)),
            ]));
        }

        let stats_para = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR))
                    .title(" 📊 Statistics ")
                    .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            );

        frame.render_widget(stats_para, chunks[1]);
        Ok(())
    }

    async fn render_configuration(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let text = vec![
            Line::from(vec![
                Span::styled("Configuration View", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Coming soon:", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("Edit interface settings", Style::default().fg(LIGHT_ORANGE)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("Configure IP addresses", Style::default().fg(LIGHT_ORANGE)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("Modify MTU and MAC", Style::default().fg(LIGHT_ORANGE)),
            ]),
            Line::from(vec![
                Span::raw("  • "),
                Span::styled("Apply profiles", Style::default().fg(LIGHT_ORANGE)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Use CLI commands for now:", Style::default().fg(TEXT_COLOR).add_modifier(Modifier::ITALIC)),
            ]),
            Line::from(vec![
                Span::styled("  netctl link set <iface> state up", Style::default().fg(SUCCESS_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("  netctl addr add <iface> <ip>/24", Style::default().fg(SUCCESS_COLOR)),
            ]),
        ];

        let config = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(BORDER_COLOR))
                    .title(" ⚙️  Configuration ")
                    .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            );

        frame.render_widget(config, area);
        Ok(())
    }

    fn render_help_overlay(&self, frame: &mut Frame) {
        let area = frame.size();

        // Center the help popup
        let popup_area = centered_rect(60, 70, area);

        let help_text = vec![
            Line::from(vec![
                Span::styled("Keyboard Shortcuts", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" / "),
                Span::styled("Shift+Tab", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Switch views", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("↑/↓", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("j/k", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Navigate interfaces", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("i", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Toggle stats bar", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("h", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("F1", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Toggle this help", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("q", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::raw(" or "),
                Span::styled("Esc", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Quit", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::styled("Ctrl+C", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
                Span::styled(" - Force exit", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Views:", Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::raw("  📊 "),
                Span::styled("Dashboard", Style::default().fg(LIGHT_ORANGE)),
                Span::styled(" - Interface list with details", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::raw("  📈 "),
                Span::styled("Statistics", Style::default().fg(LIGHT_ORANGE)),
                Span::styled(" - Network stats and gauges", Style::default().fg(TEXT_COLOR)),
            ]),
            Line::from(vec![
                Span::raw("  ⚙️  "),
                Span::styled("Configuration", Style::default().fg(LIGHT_ORANGE)),
                Span::styled(" - (Coming soon)", Style::default().fg(TEXT_COLOR)),
            ]),
        ];

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(ORANGE))
                    .title(" Help ")
                    .title_style(Style::default().fg(ORANGE).add_modifier(Modifier::BOLD)),
            )
            .style(Style::default().bg(BG_COLOR));

        frame.render_widget(Clear, popup_area);
        frame.render_widget(help, popup_area);
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

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
