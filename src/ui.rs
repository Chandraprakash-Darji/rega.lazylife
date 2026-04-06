use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Rect},
    style::{Color, Style},
    widgets::{Axis, Block, Borders, Chart, Clear, Dataset, GraphType, List, ListItem, Paragraph, Wrap},
    symbols,
    text::Span,
};

use crate::app::{App, ActiveBlock, AppMode};
use chrono::prelude::*;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn render(app: &mut App, f: &mut Frame) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.size());

    let main_area = main_chunks[0];
    let keybindings_area = main_chunks[1];

    let mut journals_state = ratatui::widgets::ListState::default();
    if app.active_block == ActiveBlock::Journals {
        journals_state.select(Some(app.selected_position));
    }
    let mut tasks_state = ratatui::widgets::ListState::default();
    if app.active_block == ActiveBlock::Tasks {
        tasks_state.select(Some(app.selected_position));
    }
    let mut habits_state = ratatui::widgets::ListState::default();
    if app.active_block == ActiveBlock::Habits {
        habits_state.select(Some(app.selected_position));
    }
    let mut moods_state = ratatui::widgets::ListState::default();
    if app.active_block == ActiveBlock::Moods {
        moods_state.select(Some(app.selected_position));
    }

    // Render based on mode
    match &app.mode {
        AppMode::Normal => {
            render_normal_view(f, app, main_area, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state);
        }
        AppMode::Focus(focused_block) => {
            render_focus_view(f, app, main_area, *focused_block, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state);
        }
        AppMode::Setup(input) => {
            let area = centered_rect(60, 20, f.size());
            let block = Block::default().title("Setup").borders(Borders::ALL);
            let text = format!("Welcome to LazyLife!\n\nPlease enter the absolute path to your data directory:\n\n{}", input);
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(paragraph, area);
        }
        AppMode::Popup(text, scroll, previous_mode) => {
            render_background(f, app, main_area, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state, previous_mode);
            let block = Block::default().title("Preview").borders(Borders::ALL);
            let paragraph = Paragraph::new(text.as_str()).block(block).wrap(Wrap { trim: true }).scroll((*scroll, 0));
            let area = centered_rect(80, 80, f.size());
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
        AppMode::EditingTask(input, _, previous_mode) => {
            render_background(f, app, main_area, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state, previous_mode);
            let area = centered_rect(60, 20, f.size());
            let block = Block::default().title("Edit Task").borders(Borders::ALL);
            let text = format!("New task name:\n\n{}", input);
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
        AppMode::EditingHabit(input, previous_mode) => {
            render_background(f, app, main_area, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state, previous_mode);
            let area = centered_rect(60, 20, f.size());
            let block = Block::default().title("New Habit").borders(Borders::ALL);
            let text = format!("New habit name:\n\n{}", input);
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
        AppMode::LoggingMood(input, previous_mode) => {
            render_background(f, app, main_area, &mut journals_state, &mut tasks_state, &mut habits_state, &mut moods_state, previous_mode);
            let area = centered_rect(60, 20, f.size());
            let block = Block::default().title("Log Mood").borders(Borders::ALL);
            let text = format!("Mood (1-5):\n\n{}", input);
            let paragraph = Paragraph::new(text).block(block);
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
        AppMode::Chart => {
            render_chart_view(f, app, main_area);
        }
        AppMode::Error(msg) => {
            let area = centered_rect(80, 20, f.size());
            let block = Block::default().title("Error").borders(Borders::ALL);
            let paragraph = Paragraph::new(msg.as_str()).block(block).wrap(Wrap { trim: true });
            f.render_widget(Clear, area);
            f.render_widget(paragraph, area);
        }
    }

    let keybindings = Paragraph::new(get_keybindings(app))
        .block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(keybindings, keybindings_area);
}

fn render_chart_view(f: &mut Frame, app: &App, area: Rect) {
    if app.moods.is_empty() {
        let block = Block::default().title("Mood Chart").borders(Borders::ALL);
        let paragraph = Paragraph::new("No mood data to display.").block(block);
        f.render_widget(paragraph, area);
        return;
    }

    let moods_data: Vec<(f64, f64)> = app
        .moods
        .iter()
        .map(|m| (m.timestamp.timestamp() as f64, m.value as f64))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Mood")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&moods_data)];

    let (min_x, max_x) = app.moods.iter().fold((f64::MAX, f64::MIN), |(min, max), m| {
        let ts = m.timestamp.timestamp() as f64;
        (min.min(ts), max.max(ts))
    });

    let x_axis = Axis::default()
        .title("Date")
        .style(Style::default().fg(Color::Gray))
        .bounds([min_x, max_x])
        .labels(vec![
            Span::from(DateTime::from_timestamp(min_x as i64, 0).unwrap().format("%Y-%m-%d").to_string()),
            Span::from(DateTime::from_timestamp(max_x as i64, 0).unwrap().format("%Y-%m-%d").to_string()),
        ]);

    let y_axis = Axis::default()
        .title("Mood")
        .style(Style::default().fg(Color::Gray))
        .bounds([1.0, 5.0])
        .labels(vec![
            Span::from(get_mood_emoji(1)),
            Span::from(get_mood_emoji(2)),
            Span::from(get_mood_emoji(3)),
            Span::from(get_mood_emoji(4)),
            Span::from(get_mood_emoji(5)),
        ]);

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Mood Chart")
                .borders(Borders::ALL),
        )
        .x_axis(x_axis)
        .y_axis(y_axis);

    f.render_widget(chart, area);
}

fn render_background(
    f: &mut Frame,
    app: &App,
    main_area: Rect,
    journals_state: &mut ratatui::widgets::ListState,
    tasks_state: &mut ratatui::widgets::ListState,
    habits_state: &mut ratatui::widgets::ListState,
    moods_state: &mut ratatui::widgets::ListState,
    previous_mode: &AppMode,
) {
    match previous_mode {
        AppMode::Focus(focused_block) => {
            render_focus_view(f, app, main_area, *focused_block, journals_state, tasks_state, habits_state, moods_state);
        }
        _ => {
            render_normal_view(f, app, main_area, journals_state, tasks_state, habits_state, moods_state);
        }
    }
}

fn render_normal_view(
    f: &mut Frame,
    app: &App,
    main_area: Rect,
    journals_state: &mut ratatui::widgets::ListState,
    tasks_state: &mut ratatui::widgets::ListState,
    habits_state: &mut ratatui::widgets::ListState,
    moods_state: &mut ratatui::widgets::ListState,
) {
    // Split into two rows
    let row_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(main_area);

    // Top row: Journals and Tasks
    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(row_chunks[0]);

    // Bottom row: Habits and Moods
    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(row_chunks[1]);

    let (journals_list, tasks_list, habits_list, moods_list) = get_lists(app);

    f.render_stateful_widget(journals_list, top_row[0], journals_state);
    f.render_stateful_widget(tasks_list, top_row[1], tasks_state);
    f.render_stateful_widget(habits_list, bottom_row[0], habits_state);
    f.render_stateful_widget(moods_list, bottom_row[1], moods_state);
}

fn render_focus_view(
    f: &mut Frame,
    app: &App,
    main_area: Rect,
    focused_block: ActiveBlock,
    journals_state: &mut ratatui::widgets::ListState,
    tasks_state: &mut ratatui::widgets::ListState,
    habits_state: &mut ratatui::widgets::ListState,
    moods_state: &mut ratatui::widgets::ListState,
) {
    let (journals_list, tasks_list, habits_list, moods_list) = get_lists(app);

    match focused_block {
        ActiveBlock::Journals => f.render_stateful_widget(journals_list, main_area, journals_state),
        ActiveBlock::Tasks => f.render_stateful_widget(tasks_list, main_area, tasks_state),
        ActiveBlock::Habits => f.render_stateful_widget(habits_list, main_area, habits_state),
        ActiveBlock::Moods => f.render_stateful_widget(moods_list, main_area, moods_state),
    }
}

fn get_mood_emoji(value: u8) -> &'static str {
    match value {
        1 => "😡",
        2 => "😟",
        3 => "😐",
        4 => "😊",
        5 => "😄",
        _ => "🤷",
    }
}

fn get_lists(app: &App) -> (List<'_>, List<'_>, List<'_>, List<'_>) {
    let mut journals_block = Block::default().borders(Borders::ALL).title("Journals");
    if app.active_block == ActiveBlock::Journals {
        journals_block = journals_block.border_style(Style::default().fg(Color::Blue));
    }
    let journals_list = List::new(app.journals.iter().map(|j| ListItem::new(j.as_str())).collect::<Vec<_>>())
        .block(journals_block)
        .highlight_style(Style::default().bg(Color::Blue));

    let mut tasks_block = Block::default().borders(Borders::ALL).title("Tasks");
    if app.active_block == ActiveBlock::Tasks {
        tasks_block = tasks_block.border_style(Style::default().fg(Color::Blue));
    }
    let tasks_list = List::new(app.tasks.iter().map(|t| {
        let status = if t.done { "[x]" } else { "[ ]" };
        ListItem::new(format!("{} {}", status, t.title))
    }).collect::<Vec<_>>())
        .block(tasks_block)
        .highlight_style(Style::default().bg(Color::Blue));

    let mut habits_block = Block::default().borders(Borders::ALL).title("Habits");
    if app.active_block == ActiveBlock::Habits {
        habits_block = habits_block.border_style(Style::default().fg(Color::Blue));
    }
    let habits_list = List::new(app.habits.iter().filter(|h| h.active).map(|h| {
        let today = Local::now().date_naive();
        let status = if h.history.contains(&today) { "[x]" } else { "[ ]" };
        ListItem::new(format!("{} {}", status, h.name))
    }).collect::<Vec<_>>())
        .block(habits_block)
        .highlight_style(Style::default().bg(Color::Blue));

    let mut moods_block = Block::default().borders(Borders::ALL).title("Moods");
    if app.active_block == ActiveBlock::Moods {
        moods_block = moods_block.border_style(Style::default().fg(Color::Blue));
    }
    let moods_list = List::new(
        app.moods
            .iter()
            .filter(|m| m.timestamp.date_naive() > Local::now().date_naive() - chrono::Duration::days(7))
            .map(|m| ListItem::new(format!("{}: {}", m.timestamp.format("%Y-%m-%d"), get_mood_emoji(m.value))))
            .collect::<Vec<_>>(),
    )
    .block(moods_block)
    .highlight_style(Style::default().bg(Color::Blue));

    (journals_list, tasks_list, habits_list, moods_list)
}

fn get_keybindings(app: &App) -> String {
    match &app.mode {
        AppMode::Normal => {
            let mut common = "[q]uit | [tab] forward | [shift+tab] backward | [f]ocus".to_string();
            if app.active_block == ActiveBlock::Moods {
                common.push_str(" | [c]hart");
            }
            let contextual = match app.active_block {
                ActiveBlock::Journals => "[n]ew | [e]dit | [v]iew",
                ActiveBlock::Tasks => "[a]dd | [e]dit | [space] toggle | [j/k] move",
                ActiveBlock::Habits => "[a]dd | [d]elete | [space] check",
                ActiveBlock::Moods => "[l]og",
            };
            format!("{} | {}", common, contextual)
        }
        AppMode::Setup(_) => "[enter] to confirm | type the path to your data directory".to_string(),
        AppMode::Popup(_, _, _) => "[q] or [esc] to close | [j/k/space] to scroll".to_string(),
        AppMode::EditingTask(_, _, _) => "[enter] to confirm | [esc] to cancel".to_string(),
        AppMode::EditingHabit(_, _) => "[enter] to confirm | [esc] to cancel".to_string(),
        AppMode::LoggingMood(_, _) => "[enter] to confirm | [esc] to cancel".to_string(),
        AppMode::Focus(focused_block) => {
            let common = "[f] or [esc] to unfocus | [j/k] to move";
            let contextual = match focused_block {
                ActiveBlock::Journals => "[n]ew | [e]dit | [v]iew",
                ActiveBlock::Tasks => "[a]dd | [e]dit | [space] toggle",
                ActiveBlock::Habits => "[a]dd | [d]elete | [space] check",
                ActiveBlock::Moods => "[l]og",
            };
            format!("{} | {}", common, contextual)
        }
        AppMode::Chart => "[q] or [esc] to exit chart view".to_string(),
        AppMode::Error(_) => "Error: Press [q] to quit or [esc] to dismiss".to_string(),
    }
}
