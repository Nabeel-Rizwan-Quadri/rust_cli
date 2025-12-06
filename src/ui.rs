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

    let data = [
        ("A", 10u64),
        ("B", 10u64),
        ("C", 10u64),
        ("D", 10u64),
    ];

    let barchart = BarChart::default()
        .block(Block::default().title("Bar Chart").borders(Borders::ALL))
        .data(&data)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    frame.render_widget(barchart, outer_layout[0]);
}
