mod app;
mod config;
mod data;
mod ui;

use std::io::{self, stdout};
use std::path::PathBuf;
use std::mem;

use app::{ActiveBlock, App, AppMode, Habit, Mood, Task};
use config::Config;
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use chrono::{Duration, Local};
use rand::Rng;


fn main() -> io::Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            let mut app = App::new();
            app.mode = AppMode::Setup("".to_string());
            run_setup(&mut terminal, &mut app)?
        }
    };

    let mut app = App::new();
    let mut db = match data::load_db(&config.data_path) {
        Ok(db) => db,
        Err(e) => {
            app.mode = AppMode::Error(format!("Failed to load database: {}", e));
            data::Database::default()
        }
    };
    // No seeding of moods or habits

    match data::read_journals(&config.data_path) {
        Ok(journals) => app.journals = journals,
        Err(e) => {
            app.mode = AppMode::Error(format!("Failed to read journals: {}", e));
        }
    }
    app.tasks = db.tasks;
    app.habits = db.habits;
    app.moods = db.moods;

    run_app(&mut terminal, app, &config)
}

fn run_setup<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<Config> {
    loop {
        terminal.draw(|f| ui::render(app, f))?;

        if let Event::Key(key) = event::read()? {
            if let AppMode::Setup(input) = &mut app.mode {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        let data_path = if input.trim().is_empty() {
                            dirs::home_dir().unwrap().join("lazylife")
                        } else {
                            PathBuf::from(input.clone())
                        };
                        let config = Config { data_path };
                        if let Err(e) = config.save() {
                            app.mode = AppMode::Error(format!("Failed to save config: {}", e));
                            return Ok(config);
                        }
                        if let Err(e) = data::init_data_dir(&config.data_path) {
                            app.mode = AppMode::Error(format!("Failed to create data directory: {}", e));
                            return Ok(config);
                        }
                        app.mode = AppMode::Normal;
                        return Ok(config);
                    }
                    _ => {}
                }
            }
        }
        break; // Temporary break for non-interactive environment
    }
    // Placeholder return, as the loop is now broken
    // In a real scenario, you'd handle the setup input and return the config
    Ok(Config { data_path: PathBuf::from("temp_data_path") })
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    config: &Config,
) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture)?;

    loop {
        terminal.draw(|f| ui::render(&mut app, f))?;

        match event::read()? {
            Event::Key(key) => match &mut app.mode {
                AppMode::Normal => {
                    match key.code {
                        KeyCode::Char('q') => {
                            let db = data::Database {
                                tasks: app.tasks.clone(),
                                habits: app.habits.clone(),
                                moods: app.moods.clone(),
                            };
                            if let Err(e) = data::write_db(&config.data_path, &db) {
                                app.mode = AppMode::Error(format!("Failed to save data: {}", e));
                                // Don't break, let user see error
                            } else {
                                break;
                            }
                        }
                        KeyCode::Char('f') => {
                            app.mode = AppMode::Focus(app.active_block);
                        }
                        KeyCode::Char('c') => {
                            use config::print_config_path;
                            use data::get_data_path;
                            let config_path = config::get_config_path().map(|p| p.display().to_string()).unwrap_or_else(|| "Not found".to_string());
                            let data_path = get_data_path().display().to_string();
                            let info = format!("Config path: {}\nApp data path: {}", config_path, data_path);
                            let previous_mode = std::mem::replace(&mut app.mode, AppMode::Normal);
                            app.mode = AppMode::Popup(info, 0, Box::new(previous_mode));
                        }
                        KeyCode::Tab => {
                            app.active_block = match app.active_block {
                                ActiveBlock::Journals => ActiveBlock::Tasks,
                                ActiveBlock::Tasks => ActiveBlock::Habits,
                                ActiveBlock::Habits => ActiveBlock::Moods,
                                ActiveBlock::Moods => ActiveBlock::Journals,
                            };
                            app.selected_position = 0;
                        }
                        KeyCode::BackTab => {
                            app.active_block = match app.active_block {
                                ActiveBlock::Journals => ActiveBlock::Moods,
                                ActiveBlock::Tasks => ActiveBlock::Journals,
                                ActiveBlock::Habits => ActiveBlock::Tasks,
                                ActiveBlock::Moods => ActiveBlock::Habits,
                            };
                            app.selected_position = 0;
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            let max_position = match app.active_block {
                                ActiveBlock::Journals => app.journals.len().saturating_sub(1),
                                ActiveBlock::Tasks => app.tasks.len().saturating_sub(1),
                                ActiveBlock::Habits => app.habits.iter().filter(|h| h.active).count().saturating_sub(1),
                                ActiveBlock::Moods => app.moods.len().saturating_sub(1),
                            };
                            if app.selected_position < max_position {
                                app.selected_position += 1;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if app.selected_position > 0 {
                                app.selected_position -= 1;
                            }
                        }
                        KeyCode::Char(c) => match app.active_block {
                            ActiveBlock::Journals => match c {
                                'n' => {
                                    let title = "New Journal".to_string();
                                    match data::create_journal(&config.data_path, &title) {
                                        Ok(filename) => app.journals.push(filename),
                                        Err(e) => app.mode = AppMode::Error(format!("Failed to create journal: {}", e)),
                                    }
                                }
                                'e' => {
                                    if let Some(journal) = app.journals.get(app.selected_position).cloned() {
                                        disable_raw_mode()?;
                                        stdout().execute(LeaveAlternateScreen)?;
                                        if let Err(e) = data::edit_journal(&config.data_path, &journal) {
                                            app.mode = AppMode::Error(format!("Failed to edit journal: {}", e));
                                        }
                                        enable_raw_mode()?;
                                        stdout().execute(EnterAlternateScreen)?;
                                        terminal.clear()?;
                                    }
                                }
                                'v' => {
                                    if let Some(journal) = app.journals.get(app.selected_position).cloned() {
                                        match data::preview_journal(&config.data_path, &journal) {
                                            Ok(content) => {
                                                let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                                app.mode = AppMode::Popup(content, 0, Box::new(previous_mode));
                                            }
                                            Err(e) => app.mode = AppMode::Error(format!("Failed to preview journal: {}", e)),
                                        }
                                    }
                                }
                                _ => {}
                            },
                            ActiveBlock::Tasks => match c {
                                'a' => {
                                    let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                    app.mode = AppMode::EditingTask("".to_string(), None, Box::new(previous_mode));
                                }
                                ' ' => {
                                    if let Some(task) = app.tasks.get_mut(app.selected_position) {
                                        task.done = !task.done;
                                    }
                                }
                                'e' => {
                                    if let Some(task) = app.tasks.get(app.selected_position) {
                                        let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                        app.mode = AppMode::EditingTask(task.title.clone(), Some(task.id), Box::new(previous_mode));
                                    }
                                }
                                _ => {}
                            },
                            ActiveBlock::Habits => match c {
                                'a' => {
                                    let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                    app.mode = AppMode::EditingHabit("".to_string(), Box::new(previous_mode));
                                }
                                 ' ' => {
                                    let active_habits: Vec<Habit> = app.habits.iter().filter(|h| h.active).cloned().collect();
                                    if let Some(selected_habit) = active_habits.get(app.selected_position) {
                                        if let Some(habit_to_modify) = app.habits.iter_mut().find(|h| h.id == selected_habit.id) {
                                            let today = Local::now().date_naive();
                                            if let Some(pos) = habit_to_modify.history.iter().position(|&d| d == today) {
                                                habit_to_modify.history.remove(pos);
                                            } else {
                                                habit_to_modify.history.push(today);
                                            }
                                        }
                                    }
                                }
                                'd' => {
                                    let active_habits: Vec<Habit> = app.habits.iter().filter(|h| h.active).cloned().collect();
                                    if let Some(selected_habit) = active_habits.get(app.selected_position) {
                                        if let Some(habit_to_modify) = app.habits.iter_mut().find(|h| h.id == selected_habit.id) {
                                            habit_to_modify.active = false;
                                        }
                                    }
                                }
                                _ => {}
                            },
                            ActiveBlock::Moods => match c {
                                'l' => {
                                    let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                    app.mode = AppMode::LoggingMood("".to_string(), Box::new(previous_mode));
                                }
                                _ => {}
                            },
                        },
                        _ => {}
                    }
                }
                AppMode::Focus(focused_block) => match key.code {
                    KeyCode::Char('f') | KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        let max_position = match focused_block {
                            ActiveBlock::Journals => app.journals.len().saturating_sub(1),
                            ActiveBlock::Tasks => app.tasks.len().saturating_sub(1),
                            ActiveBlock::Habits => app.habits.iter().filter(|h| h.active).count().saturating_sub(1),
                            ActiveBlock::Moods => app.moods.len().saturating_sub(1),
                        };
                        if app.selected_position < max_position {
                            app.selected_position += 1;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if app.selected_position > 0 {
                            app.selected_position -= 1;
                        }
                    }
                    KeyCode::Char(c) => match *focused_block {
                        ActiveBlock::Journals => match c {
                            'n' => {
                                let title = "New Journal".to_string();
                                match data::create_journal(&config.data_path, &title) {
                                    Ok(filename) => app.journals.push(filename),
                                    Err(e) => app.mode = AppMode::Error(format!("Failed to create journal: {}", e)),
                                }
                            }
                            'e' => {
                                if let Some(journal) = app.journals.get(app.selected_position).cloned() {
                                    disable_raw_mode()?;
                                    stdout().execute(LeaveAlternateScreen)?;
                                    if let Err(e) = data::edit_journal(&config.data_path, &journal) {
                                        app.mode = AppMode::Error(format!("Failed to edit journal: {}", e));
                                    }
                                    enable_raw_mode()?;
                                    stdout().execute(EnterAlternateScreen)?;
                                    terminal.clear()?;
                                }
                            }
                            'v' => {
                                if let Some(journal) = app.journals.get(app.selected_position).cloned() {
                                    match data::preview_journal(&config.data_path, &journal) {
                                        Ok(content) => {
                                            let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                            app.mode = AppMode::Popup(content, 0, Box::new(previous_mode));
                                        }
                                        Err(e) => app.mode = AppMode::Error(format!("Failed to preview journal: {}", e)),
                                    }
                                }
                            }
                            _ => {}
                        },
                        ActiveBlock::Tasks => match c {
                            'a' => {
                                let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                app.mode = AppMode::EditingTask("".to_string(), None, Box::new(previous_mode));
                            }
                            ' ' => {
                                if let Some(task) = app.tasks.get_mut(app.selected_position) {
                                    task.done = !task.done;
                                }
                            }
                            'e' => {
                                if let Some(task) = app.tasks.get(app.selected_position) {
                                    let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                    app.mode = AppMode::EditingTask(task.title.clone(), Some(task.id), Box::new(previous_mode));
                                }
                            }
                            _ => {}
                        },
                        ActiveBlock::Habits => match c {
                            'a' => {
                                let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                app.mode = AppMode::EditingHabit("".to_string(), Box::new(previous_mode));
                            }
                            'c' | ' ' => {
                                let active_habits: Vec<Habit> = app.habits.iter().filter(|h| h.active).cloned().collect();
                                if let Some(selected_habit) = active_habits.get(app.selected_position) {
                                    if let Some(habit_to_modify) = app.habits.iter_mut().find(|h| h.id == selected_habit.id) {
                                        let today = Local::now().date_naive();
                                        if let Some(pos) = habit_to_modify.history.iter().position(|&d| d == today) {
                                            habit_to_modify.history.remove(pos);
                                        } else {
                                            habit_to_modify.history.push(today);
                                        }
                                    }
                                }
                            }
                            'd' => {
                                let active_habits: Vec<Habit> = app.habits.iter().filter(|h| h.active).cloned().collect();
                                if let Some(selected_habit) = active_habits.get(app.selected_position) {
                                    if let Some(habit_to_modify) = app.habits.iter_mut().find(|h| h.id == selected_habit.id) {
                                        habit_to_modify.active = false;
                                    }
                                }
                            }
                            _ => {}
                        },
                        ActiveBlock::Moods => match c {
                            'l' => {
                                let previous_mode = mem::replace(&mut app.mode, AppMode::Normal);
                                app.mode = AppMode::LoggingMood("".to_string(), Box::new(previous_mode));
                            }
                            _ => {}
                        },
                    },
                    _ => {}
                },
                AppMode::Popup(_, scroll, previous_mode) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.mode = *previous_mode.clone();
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        *scroll = scroll.saturating_add(1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        *scroll = scroll.saturating_sub(1);
                    }
                    KeyCode::Char(' ') => {
                        *scroll = scroll.saturating_add(10);
                    }
                    _ => {}
                },
                AppMode::EditingTask(input, id, previous_mode) => match key.code {
                    KeyCode::Enter => {
                        if let Some(task_id) = id {
                            if let Some(task) = app.tasks.iter_mut().find(|t| t.id == *task_id) {
                                task.title = input.clone();
                            }
                        } else {
                            let new_task = Task {
                                id: app.tasks.len() as u64 + 1,
                                title: input.clone(),
                                done: false,
                            };
                            app.tasks.push(new_task);
                            app.selected_position = app.tasks.len() - 1;
                        }
                        app.mode = *previous_mode.clone();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        app.mode = *previous_mode.clone();
                    }
                    _ => {}
                },
                AppMode::EditingHabit(input, previous_mode) => match key.code {
                    KeyCode::Enter => {
                        let new_habit = Habit {
                            id: app.habits.len() as u64 + 1,
                            name: input.clone(),
                            history: vec![],
                            active: true,
                        };
                        app.habits.push(new_habit);
                        app.mode = *previous_mode.clone();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        app.mode = *previous_mode.clone();
                    }
                    _ => {}
                },
                AppMode::LoggingMood(input, previous_mode) => match key.code {
                    KeyCode::Enter => {
                        if let Ok(value) = input.parse::<u8>() {
                            if (1..=5).contains(&value) {
                                let today = chrono::Local::now().date_naive();
                                if let Some(mood) = app.moods.iter_mut().find(|m| m.timestamp.date_naive() == today) {
                                    mood.value = value;
                                } else {
                                    let new_mood = Mood {
                                        id: app.moods.len() as u64 + 1,
                                        value,
                                        timestamp: chrono::Local::now(),
                                    };
                                    app.moods.push(new_mood);
                                }
                            }
                        }
                        app.mode = *previous_mode.clone();
                    }
                    KeyCode::Char(c) => {
                        if c.is_digit(10) {
                            input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => {
                        app.mode = *previous_mode.clone();
                    }
                    _ => {}
                },
                AppMode::Chart => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            },
            Event::Mouse(mouse) => {
                if let AppMode::Popup(_, scroll, _) = &mut app.mode {
                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            *scroll = scroll.saturating_sub(1);
                        }
                        MouseEventKind::ScrollDown => {
                            *scroll = scroll.saturating_add(1);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // restore terminal
    disable_raw_mode()?;
    crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}