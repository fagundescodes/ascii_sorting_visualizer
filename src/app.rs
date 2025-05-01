use rand::{rng, seq::SliceRandom};

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

#[derive(Debug, Clone)]
pub struct AppState {
    pub array: Vec<i32>,
    pub algorithm: SortAlgorithm,
    pub step_count: usize,
}

impl AppState {
    pub fn new() -> Self {
        let mut rng = rng();
        let mut numbers: Vec<i32> = (1..=20).collect();
        numbers.shuffle(&mut rng);
        let array = numbers[0..10].to_vec();
        Self {
            array,
            algorithm: SortAlgorithm::Bubble,
            step_count: 0,
        }
    }
}

pub struct App {
    pub state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }
}
