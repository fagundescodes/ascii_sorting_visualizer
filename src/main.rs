use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, stdout};

mod sort;
mod ui;
mod app;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let mut rng = rand::rng();
    // let mut numbers: Vec<i32> = (1..=20).collect();
    // numbers.shuffle(&mut rng);
    // let mut my_array: Vec<i32> = numbers[0..10].to_vec();

    let mut app = app::App::new();
    let mut array = app.state.array.clone();
    sort::bubble_sort(&mut array, &mut terminal)?;
    // sort::insertion_sort(&mut my_array);
    // sort::selection_sort(&mut my_array);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
