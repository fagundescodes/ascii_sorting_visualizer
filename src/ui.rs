use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use ratatui::{
    Terminal,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    io::{self, stdout},
    thread::sleep,
    time::Duration,
};

use crate::app::AppState;

pub fn render(
    state: &AppState,
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> io::Result<()> {
    terminal.draw(|f| {
        let text = format!(
            "{} - Steps: {}\nArray: {:?}",
            state.algorithm.name(),
            state.step_count,
            state.array
        );
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Sorting Visualizer")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White));
        f.render_widget(paragraph, f.area());
    })?;
    sleep(Duration::from_millis(300));
    Ok(())
}

pub fn clear_log() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}
