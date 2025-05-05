use crate::app::AppState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn ui(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(f.area());

    let title = Paragraph::new(format!(
        "Sorting Visualizer - {} - Steps: {}",
        app_state.algorithm.name(),
        app_state.step_count
    ))
    .style(Style::default().fg(Color::Cyan))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let bars_area = chunks[1];
    render_bars(f, bars_area, app_state);

    let controls = format!(
        "Controls: Space = {}, r = Randomize, a = Change Algorithm",
        match app_state.state {
            crate::app::SortingState::Ready => "Start",
            crate::app::SortingState::Sorting => "Pause",
            crate::app::SortingState::Paused => "Resume",
            crate::app::SortingState::Done => "Start",
        }
    );
    let controls_widget = Paragraph::new(controls)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(controls_widget, chunks[2]);
}

pub fn render_bars(f: &mut Frame, area: Rect, app_state: &AppState) {
    let max_value = app_state.array.iter().max().copied().unwrap_or(1);
    let bar_width = (area.width as usize / app_state.array.len()).max(1) as u16;

    for (i, &value) in app_state.array.iter().enumerate() {
        let height = ((value as f64 / max_value as f64) * (area.height as f64 - 2.0)) as u16;
        let bar_area = Rect {
            x: area.x + (i as u16 * bar_width),
            y: area.y + area.height - 1 - height,
            width: bar_width.min(area.width - (i as u16 * bar_width)),
            height,
        };

        let color = if app_state.highlighted_indices.0 == Some(i)
            || app_state.highlighted_indices.1 == Some(i)
        {
            Color::Red
        } else if app_state.state == crate::app::SortingState::Done {
            Color::Green
        } else {
            Color::Blue
        };

        let bar = Block::default().style(Style::default().bg(color));
        f.render_widget(bar, bar_area);

        if bar_width >= 3 {
            let label = Paragraph::new(value.to_string()).style(Style::default().fg(Color::White));
            let label_area = Rect {
                x: area.x + (i as u16 * bar_width),
                y: area.y + area.height - 1,
                width: bar_width,
                height: 1,
            };
            f.render_widget(label, label_area);
        }
    }
}
