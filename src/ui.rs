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

pub fn render(
    arr: &[i32],
    visual_1: Option<usize>,
    visual_2: Option<usize>,
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> io::Result<()> {
    terminal.draw(|f| {
        let text = format!("Array: {:?}", arr);
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
