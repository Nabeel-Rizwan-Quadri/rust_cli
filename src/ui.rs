use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::{Color, Style, Stylize}, text::ToSpan, widgets::{BarChart, Block, Borders, Widget}};

pub fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Min(1)])
        .split(frame.area());

    Block::bordered()
        .fg(Color::Green)
        .title(" My Blocky Boy ".to_span().into_centered_line())
        .render(outer_layout[0], frame.buffer_mut());

    // Read from shared CLIENT_DATA
    let client_data = crate::CLIENT_DATA.lock().unwrap();

    let data: Vec<(&str, u64)> = if client_data.is_empty() {
        // Default data when no client has sent data yet
        vec![("A", 0), ("B", 0), ("C", 0), ("D", 0)]
    } else {
        client_data.as_vec()
    };

    let barchart = BarChart::default()
        .block(Block::default()
            .title("Bar Chart - Real-time Client Data")
            .borders(Borders::ALL))
        .data(&data)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    frame.render_widget(barchart, outer_layout[0]);
}
