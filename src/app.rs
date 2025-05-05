use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::{rng, seq::SliceRandom};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

use crate::sort::{bubble_sort, insertion_sort, selection_sort};
use crate::ui::ui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortAlgorithm {
    Bubble,
    Selection,
    Insertion,
}

impl SortAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            SortAlgorithm::Bubble => "Bubble Sort",
            SortAlgorithm::Selection => "Selection Sort",
            SortAlgorithm::Insertion => "Insertion Sort",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortingState {
    Ready,
    Sorting,
    Paused,
    Done,
}

#[derive(Debug, Clone)]
pub enum SortMessage {
    Continue,
    Pause,
    Stop,
}

#[derive(Debug, Clone)]
pub enum SortEvent {
    Comparing(usize, usize),
    Swapping(usize, usize),
    Highlighting(usize),
    Done,
    StepIncrement,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub array: Vec<u32>,
    pub algorithm: SortAlgorithm,
    pub state: SortingState,
    pub highlighted_indices: (Option<usize>, Option<usize>),
    pub step_count: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            array: Self::generate_random_array(),
            algorithm: SortAlgorithm::Bubble,
            state: SortingState::Ready,
            highlighted_indices: (None, None),
            step_count: 0,
        }
    }
}

impl AppState {
    pub fn generate_random_array() -> Vec<u32> {
        let mut rng = rng();
        let mut numbers: Vec<u32> = (1..=20).collect();
        numbers.shuffle(&mut rng);
        numbers.into_iter().take(10).collect()
    }

    pub fn randomize_array(&mut self) {
        self.array = Self::generate_random_array();
        self.state = SortingState::Ready;
        self.step_count = 0;
        self.highlighted_indices = (None, None);
    }

    pub fn next_algorithm(&mut self) {
        self.algorithm = match self.algorithm {
            SortAlgorithm::Bubble => SortAlgorithm::Selection,
            SortAlgorithm::Selection => SortAlgorithm::Insertion,
            SortAlgorithm::Insertion => SortAlgorithm::Bubble,
        };
        self.state = SortingState::Ready;
        self.step_count = 0;
    }
}

pub struct App {
    pub state: Arc<Mutex<AppState>>,
    pub should_quit: bool,
    pub sort_tx: Option<mpsc::Sender<SortMessage>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState::default())),
            should_quit: false,
            sort_tx: None,
        }
    }

    pub fn run_ui(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let tick_rate = Duration::from_millis(50);
        let mut last_tick = std::time::Instant::now();

        let (event_tx, mut event_rx) = mpsc::channel::<SortEvent>(100);

        loop {
            while let Ok(event) = event_rx.try_recv() {
                self.handle_sort_event(event);
            }

            terminal.draw(|f| ui(f, &self.state.lock().unwrap()))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code, event_tx.clone());
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = std::time::Instant::now();
            }

            if self.should_quit {
                if let Some(tx) = &self.sort_tx {
                    let _ = tx.try_send(SortMessage::Stop);
                }
                break;
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    pub fn handle_sort_event(&mut self, event: SortEvent) {
        let mut app_state = self.state.lock().unwrap();
        match event {
            SortEvent::Comparing(i, j) => {
                app_state.highlighted_indices = (Some(i), Some(j));
            }
            SortEvent::Swapping(i, j) => {
                app_state.array.swap(i, j);
                app_state.highlighted_indices = (Some(i), Some(j));
            }
            SortEvent::Highlighting(i) => {
                app_state.highlighted_indices = (Some(i), None);
            }
            SortEvent::Done => {
                app_state.state = SortingState::Done;
                app_state.highlighted_indices = (None, None);
                self.sort_tx = None;
            }
            SortEvent::StepIncrement => {
                app_state.step_count += 1;
            }
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, event_tx: mpsc::Sender<SortEvent>) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') => {
                let mut app_state = self.state.lock().unwrap();
                match app_state.state {
                    SortingState::Ready => {
                        let algorithm = app_state.algorithm;
                        let array = app_state.array.clone();
                        app_state.state = SortingState::Sorting;
                        drop(app_state);
                        self.start_sorting(algorithm, array, event_tx);
                    }
                    SortingState::Sorting => {
                        if let Some(tx) = &self.sort_tx {
                            let _ = tx.try_send(SortMessage::Pause);
                            app_state.state = SortingState::Paused;
                        }
                    }
                    SortingState::Paused => {
                        if let Some(tx) = &self.sort_tx {
                            let _ = tx.try_send(SortMessage::Continue);
                            app_state.state = SortingState::Sorting;
                        }
                    }
                    SortingState::Done => {}
                }
            }
            KeyCode::Char('r') => {
                if let Some(tx) = &self.sort_tx {
                    let _ = tx.try_send(SortMessage::Stop);
                    self.sort_tx = None;
                }
                let mut app_state = self.state.lock().unwrap();
                app_state.randomize_array();
            }
            KeyCode::Char('a') => {
                let app_state = self.state.lock().unwrap();
                if app_state.state == SortingState::Ready || app_state.state == SortingState::Done {
                    drop(app_state);
                    if let Some(tx) = &self.sort_tx {
                        let _ = tx.try_send(SortMessage::Stop);
                        self.sort_tx = None;
                    }
                    let mut app_state = self.state.lock().unwrap();
                    app_state.next_algorithm();
                }
            }
            _ => {}
        }
    }

    pub fn start_sorting(
        &mut self,
        algorithm: SortAlgorithm,
        array: Vec<u32>,
        event_tx: mpsc::Sender<SortEvent>,
    ) {
        let (sort_tx, mut sort_rx) = mpsc::channel::<SortMessage>(10);
        self.sort_tx = Some(sort_tx);
        {
            let mut app_state = self.state.lock().unwrap();
            app_state.state = SortingState::Sorting;
        }
        tokio::spawn(async move {
            match algorithm {
                SortAlgorithm::Bubble => bubble_sort(array, event_tx, &mut sort_rx).await,
                SortAlgorithm::Selection => selection_sort(array, event_tx, &mut sort_rx).await,
                SortAlgorithm::Insertion => insertion_sort(array, event_tx, &mut sort_rx).await,
            }
        });
    }
}
