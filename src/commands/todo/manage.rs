use crate::cli::args::TodoAction;
use crate::core::{CldevError, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Todo item structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub description: String,
    pub status: TodoStatus,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub related_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Todo list structure
#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    pub metadata: HashMap<String, String>,
}

impl TodoList {
    /// Create a new empty todo list
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Load todo list from file
    pub fn load() -> Result<Self> {
        let path = Self::get_todo_file_path()?;

        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&path)?;
        let todo_list: TodoList = serde_json::from_str(&content)?;

        Ok(todo_list)
    }

    /// Save todo list to file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_todo_file_path()?;

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;

        Ok(())
    }

    /// Get todo file path
    fn get_todo_file_path() -> Result<PathBuf> {
        // Try project-level first
        let project_path = PathBuf::from(".cldev/todos.json");
        if project_path.parent().map(|p| p.exists()).unwrap_or(false) {
            return Ok(project_path);
        }

        // Fall back to global
        let home =
            dirs::home_dir().ok_or_else(|| CldevError::config("Failed to get home directory"))?;

        let global_dir = home.join(".claude").join("todos");
        if !global_dir.exists() {
            fs::create_dir_all(&global_dir)?;
        }

        Ok(global_dir.join("todos.json"))
    }

    /// Add a new todo item
    pub fn add_todo(&mut self, description: String, priority: Priority) -> String {
        let id = format!("todo_{}", chrono::Local::now().format("%Y%m%d_%H%M%S"));
        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let todo = TodoItem {
            id: id.clone(),
            description,
            status: TodoStatus::Pending,
            created_at,
            completed_at: None,
            priority,
            tags: Vec::new(),
            related_files: Vec::new(),
        };

        self.todos.push(todo);
        id
    }

    /// Mark todo as completed
    pub fn complete_todo(&mut self, id: &str) -> Result<()> {
        let todo = self
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| CldevError::command(format!("Todo not found: {}", id)))?;

        todo.status = TodoStatus::Completed;
        todo.completed_at = Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        Ok(())
    }

    /// Get pending todos
    pub fn get_pending(&self) -> Vec<&TodoItem> {
        self.todos
            .iter()
            .filter(|t| matches!(t.status, TodoStatus::Pending | TodoStatus::InProgress))
            .collect()
    }

    /// Sync with git commits
    pub fn sync_with_git(&mut self) -> Result<()> {
        use std::process::Command;

        // Get recent commits
        let output = Command::new("git")
            .args(["log", "--oneline", "-10"])
            .output()?;

        if !output.status.success() {
            return Err(CldevError::command("Failed to read git log"));
        }

        let commits = String::from_utf8_lossy(&output.stdout);

        // Auto-complete todos that match commit messages
        for line in commits.lines() {
            for todo in &mut self.todos {
                if matches!(todo.status, TodoStatus::Pending | TodoStatus::InProgress) {
                    let desc_lower = todo.description.to_lowercase();
                    let commit_lower = line.to_lowercase();

                    // Simple matching: if commit contains todo description keywords
                    if commit_lower.contains(&desc_lower) {
                        todo.status = TodoStatus::Completed;
                        todo.completed_at =
                            Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
                    }
                }
            }
        }

        Ok(())
    }
}

/// Handle todo manage command
pub fn handle_manage(action: TodoAction, description: Option<String>) -> Result<()> {
    match action {
        TodoAction::Add => add_todo(description)?,
        TodoAction::List => list_todos()?,
        TodoAction::Complete => complete_todo()?,
        TodoAction::Sync => sync_todos()?,
        TodoAction::Interactive => interactive_mode()?,
    }

    Ok(())
}

/// Add a new todo
fn add_todo(description: Option<String>) -> Result<()> {
    println!("{}", "➕ Add New Todo".cyan().bold());

    let mut todo_list = TodoList::load()?;

    // Get description
    let desc = match description {
        Some(d) => d,
        None => Input::new()
            .with_prompt("Todo description")
            .interact_text()?,
    };

    // Get priority
    let priorities = vec!["Low", "Medium", "High", "Critical"];
    let priority_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Priority")
        .items(&priorities)
        .default(1)
        .interact()?;

    let priority = match priority_idx {
        0 => Priority::Low,
        1 => Priority::Medium,
        2 => Priority::High,
        3 => Priority::Critical,
        _ => Priority::Medium,
    };

    // Add todo
    let id = todo_list.add_todo(desc.clone(), priority);
    todo_list.save()?;

    println!("{}", "\n✅ Todo added successfully!".green());
    println!("{} ID: {}", "ℹ️".cyan(), id.yellow());
    println!("{} Description: {}", "ℹ️".cyan(), desc);

    Ok(())
}

/// List all todos
fn list_todos() -> Result<()> {
    println!("{}", "📋 Todo List".cyan().bold());

    let todo_list = TodoList::load()?;
    let pending = todo_list.get_pending();

    if pending.is_empty() {
        println!("{}", "\n✅ No pending todos!".green());
        return Ok(());
    }

    println!("\n{} {} pending todo(s)", "ℹ️".cyan(), pending.len());

    // Display todos
    for (i, todo) in pending.iter().enumerate() {
        display_todo(todo, i + 1);
    }

    // Summary by priority
    display_priority_summary(&pending);

    Ok(())
}

/// Complete a todo
fn complete_todo() -> Result<()> {
    println!("{}", "✅ Complete Todo".cyan().bold());

    let mut todo_list = TodoList::load()?;
    let pending = todo_list.get_pending();

    if pending.is_empty() {
        println!("{}", "\n✅ No pending todos!".green());
        return Ok(());
    }

    // Create selection list
    let items: Vec<String> = pending
        .iter()
        .map(|t| format!("{} - {:?}", t.description, t.priority))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select todo to complete")
        .items(&items)
        .interact()?;

    let selected_todo = pending[selection];
    let todo_id = selected_todo.id.clone();
    let todo_desc = selected_todo.description.clone();

    todo_list.complete_todo(&todo_id)?;
    todo_list.save()?;

    println!("\n{} Todo completed: {}", "✅".green(), todo_desc);

    Ok(())
}

/// Sync todos with git
fn sync_todos() -> Result<()> {
    println!("{}", "🔄 Syncing todos with git...".cyan().bold());

    let mut todo_list = TodoList::load()?;
    let before_count = todo_list.get_pending().len();

    todo_list.sync_with_git()?;

    let after_count = todo_list.get_pending().len();
    let completed = before_count - after_count;

    todo_list.save()?;

    println!("\n{} Sync completed!", "✅".green());
    println!(
        "{} {} todo(s) auto-completed based on git commits",
        "ℹ️".cyan(),
        completed
    );

    Ok(())
}

/// Interactive todo management mode
fn interactive_mode() -> Result<()> {
    println!("{}", "🎯 Interactive Todo Management".cyan().bold());

    loop {
        let actions = vec![
            "📋 List todos",
            "➕ Add todo",
            "✅ Complete todo",
            "🔄 Sync with git",
            "🚪 Exit",
        ];

        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .items(&actions)
            .interact()?;

        match choice {
            0 => list_todos()?,
            1 => add_todo(None)?,
            2 => complete_todo()?,
            3 => sync_todos()?,
            4 => {
                println!("{}", "👋 Goodbye!".cyan());
                break;
            }
            _ => {}
        }

        println!();
    }

    Ok(())
}

/// Display a single todo
fn display_todo(todo: &TodoItem, index: usize) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};

    println!("\n{} Todo #{}", "📌".cyan(), index);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    // Status
    let status_cell = match todo.status {
        TodoStatus::Pending => Cell::new("Pending").fg(Color::Yellow),
        TodoStatus::InProgress => Cell::new("In Progress").fg(Color::Cyan),
        TodoStatus::Completed => Cell::new("Completed").fg(Color::Green),
    };

    // Priority
    let priority_cell = match todo.priority {
        Priority::Critical => Cell::new("Critical").fg(Color::Red),
        Priority::High => Cell::new("High").fg(Color::Yellow),
        Priority::Medium => Cell::new("Medium").fg(Color::Cyan),
        Priority::Low => Cell::new("Low").fg(Color::Green),
    };

    table.add_row(vec![Cell::new("Description"), Cell::new(&todo.description)]);
    table.add_row(vec![Cell::new("Status"), status_cell]);
    table.add_row(vec![Cell::new("Priority"), priority_cell]);
    table.add_row(vec![Cell::new("Created"), Cell::new(&todo.created_at)]);

    if !todo.tags.is_empty() {
        table.add_row(vec![Cell::new("Tags"), Cell::new(todo.tags.join(", "))]);
    }

    println!("{}", table);
}

/// Display priority summary
fn display_priority_summary(todos: &[&TodoItem]) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut critical = 0;
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;

    for todo in todos {
        match todo.priority {
            Priority::Critical => critical += 1,
            Priority::High => high += 1,
            Priority::Medium => medium += 1,
            Priority::Low => low += 1,
        }
    }

    println!("\n{}", "📊 Priority Summary".green().bold());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Priority", "Count"]);

    if critical > 0 {
        table.add_row(vec!["Critical", &critical.to_string()]);
    }
    if high > 0 {
        table.add_row(vec!["High", &high.to_string()]);
    }
    if medium > 0 {
        table.add_row(vec!["Medium", &medium.to_string()]);
    }
    if low > 0 {
        table.add_row(vec!["Low", &low.to_string()]);
    }

    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let mut list = TodoList::new();
        let id = list.add_todo("Test todo".to_string(), Priority::Medium);

        assert_eq!(list.todos.len(), 1);
        assert_eq!(list.todos[0].description, "Test todo");
        assert!(id.starts_with("todo_"));
    }

    #[test]
    fn test_todo_completion() {
        let mut list = TodoList::new();
        let id = list.add_todo("Test todo".to_string(), Priority::Medium);

        list.complete_todo(&id).unwrap();
        assert_eq!(list.todos[0].status, TodoStatus::Completed);
        assert!(list.todos[0].completed_at.is_some());
    }
}
