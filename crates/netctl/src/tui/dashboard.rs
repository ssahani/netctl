use miette::Result;
use netctl_core::NetworkManager;
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, SystemTime};

pub struct Dashboard {
    manager: NetworkManager,
    last_update: SystemTime,
    refresh_rate: Duration,
}

impl Dashboard {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            manager: NetworkManager::new().await?,
            last_update: SystemTime::now(),
            refresh_rate: Duration::from_secs(1),
        })
    }

    pub fn should_refresh(&self) -> bool {
        self.last_update.elapsed().unwrap_or_default() >= self.refresh_rate
    }

    pub fn mark_refreshed(&mut self) {
        self.last_update = SystemTime::now();
    }

    pub async fn render(&mut self, frame: &mut Frame<'_>) -> Result<()> {
        let area = frame.size();

        // Create main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        // Render title
        self.render_title(frame, chunks[0]);

        // Render interface list
        self.render_interfaces(frame, chunks[1]).await?;

        // Render footer
        self.render_footer(frame, chunks[2]);

        Ok(())
    }

    fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Cyan));

        let title_text = Paragraph::new("🌐 netctl - Real-time Network Dashboard")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(title);

        frame.render_widget(title_text, area);
    }

    async fn render_interfaces(&self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        // Get interface information
        let interfaces = match self.manager.list_links().await {
            Ok(links) => links,
            Err(_) => Vec::new(),
        };

        // Create table header
        let header = Row::new(vec!["Index", "Name", "State", "MTU", "MAC Address"])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1);

        // Create table rows
        let rows: Vec<Row> = interfaces
            .iter()
            .map(|iface| {
                let state_color = match iface.state {
                    netctl_types::network::LinkState::Up => Color::Green,
                    netctl_types::network::LinkState::Down => Color::Red,
                };

                Row::new(vec![
                    iface.index.to_string(),
                    iface.name.clone(),
                    format!("{:?}", iface.state),
                    iface.mtu.to_string(),
                    iface.mac_address.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "-".to_string()),
                ])
                .style(Style::default().fg(state_color))
            })
            .collect();

        // Create table
        let table = Table::new(
            rows,
            [
                Constraint::Length(6),
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
                .title(format!(" Interfaces ({}) ", interfaces.len()))
                .title_style(Style::default().fg(Color::Green)),
        )
        .column_spacing(2);

        frame.render_widget(table, area);
        Ok(())
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new("Press 'q' to quit | Auto-refresh: 1s")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        frame.render_widget(footer, area);
    }
}
