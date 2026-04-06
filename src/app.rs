use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: u64,
    pub name: String,
    pub history: Vec<NaiveDate>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    pub id: u64,
    pub value: u8,
    pub timestamp: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveBlock {
    Journals,
    Tasks,
    Habits,
    Moods,
}

#[derive(Clone)]
pub enum AppMode {
    Normal,
    Setup(String),
    Popup(String, u16, Box<AppMode>),
    EditingTask(String, Option<u64>, Box<AppMode>),
    EditingHabit(String, Box<AppMode>),
    LoggingMood(String, Box<AppMode>),
    Focus(ActiveBlock),
    Chart,
    Error(String),
}

pub struct App {
    pub journals: Vec<String>,
    pub tasks: Vec<Task>,
    pub habits: Vec<Habit>,
    pub moods: Vec<Mood>,
    pub active_block: ActiveBlock,
    pub selected_position: usize,
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Self {
        Self {
            journals: vec![],
            tasks: vec![],
            habits: vec![],
            moods: vec![],
            active_block: ActiveBlock::Journals,
            selected_position: 0,
            mode: AppMode::Normal,
        }
    }
}
