use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{BarChart, Block, Borders, Paragraph, Widget, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(frame.size());

    // Read from shared CLIENT_DATA
    let client_data = crate::CLIENT_DATA.lock().unwrap();

    let data: Vec<(&str, u64)> = if client_data.is_empty() {
        // Default data when no client has sent data yet
        vec![("A", 0), ("B", 0), ("C", 0), ("D", 0)]
    } else {
        client_data.as_vec()
    };

    let barchart = BarChart::default()
        .block(
            Block::default()
                .title("Bar Chart - Real-time Client Data")
                .borders(Borders::ALL),
        )
        .data(&data)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    frame.render_widget(barchart, chunks[0]);

    // Store height for scroll calculation
    *crate::LOG_VIEW_HEIGHT.lock().unwrap() = chunks[1].height;

    let server_logs = crate::SERVER_LOGS.lock().unwrap();
    let log_lines: Vec<Line> = server_logs.iter().map(|s| Line::from(s.clone())).collect();
    let scroll = *crate::SCROLL_STATE.lock().unwrap();
    let log_paragraph = Paragraph::new(log_lines)
        .block(Block::default().title("Server Logs").borders(Borders::ALL))
        .wrap(Wrap { trim: true })
        .scroll(scroll);

    frame.render_widget(log_paragraph, chunks[1]);
}
