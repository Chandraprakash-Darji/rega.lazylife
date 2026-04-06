use crate::app::{Task, Habit, Mood};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use crate::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Database {
    pub tasks: Vec<Task>,
    pub habits: Vec<Habit>,
    pub moods: Vec<Mood>,
}

fn command_exists(command: &str) -> bool {
    Command::new("which").arg(command).status().map_or(false, |s| s.success())
}

pub fn init_data_dir(data_path: &Path) -> std::io::Result<()> {
    fs::create_dir_all(data_path.join("journals"))?;
    let db_path = data_path.join("db.json");
    if !db_path.exists() {
        let db = Database::default();
        let content = serde_json::to_string_pretty(&db)?;
        fs::write(db_path, content)?;
    }
    Ok(())
}
// Returns the default data path: ~/lazylife
pub fn get_data_path() -> PathBuf {
    dirs::home_dir().unwrap().join("lazylife")
}

pub fn create_journal(data_path: &Path, title: &str) -> std::io::Result<String> {
    let datetime = chrono::Local::now();
    let mut counter = 0;
    let filename = loop {
        let formatted_title = title.replace(" ", "_");
        let suffix = if counter == 0 {
            String::new()
        } else {
            format!("-{:02}", counter)
        };
        let filename = format!(
            "{}-{}-{}{}.md",
            datetime.format("%Y-%m-%d"),
            datetime.format("%H%M"),
            formatted_title,
            suffix
        );
        let full_path = data_path.join("journals").join(&filename);
        if !full_path.exists() {
            break filename;
        }
        counter += 1;
    };

    let full_path = data_path.join("journals").join(&filename);
    let mut file = fs::File::create(&full_path)?;
    file.write_all(format!("# {}

", title).as_bytes())?;
    Ok(filename)
}

pub fn edit_journal(data_path: &Path, filename: &str) -> std::io::Result<()> {
    let path = data_path.join("journals").join(filename);
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
        if command_exists("nvim") {
            "nvim".to_string()
        } else {
            "vim".to_string()
        }
    });
    Command::new(editor).arg(path).status()?;
    Ok(())
}

pub fn preview_journal(data_path: &Path, filename: &str) -> std::io::Result<String> {
    let path = data_path.join("journals").join(filename);
    fs::read_to_string(path)
}

pub fn read_journals(data_path: &Path) -> std::io::Result<Vec<String>> {
    let journals_dir = data_path.join("journals");
    if !journals_dir.exists() {
        fs::create_dir_all(&journals_dir)?;
    }
    let mut journals = Vec::new();
    for entry in fs::read_dir(&journals_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                journals.push(filename.to_string());
            }
        }
    }
    Ok(journals)
}

pub fn load_db(data_path: &Path) -> Result<Database, Box<dyn std::error::Error>> {
    let db_path = data_path.join("db.json");
    let content = fs::read_to_string(db_path)?;
    let db: Database = serde_json::from_str(&content)?;
    Ok(db)
}

pub fn write_db(data_path: &Path, db: &Database) -> std::io::Result<()> {
    let content = serde_json::to_string_pretty(db)?;
    fs::write(data_path.join("db.json"), content)?;
    Ok(())
}