use std::slice::Iter;
use chrono::prelude::*;
use eframe::egui;

#[derive(Clone)]
pub struct Task {
    pub summary: String,
    pub completed: bool,
    pub description: String,
    pub progress: u8,
    pub priority: u8,
    pub status: String,
    pub due: NaiveDate
}

// Define default task
impl Default for Task {
    fn default () -> Self {
        Self {
            summary: "Do the dishes".to_string(),
            completed: false,
            description: "".to_string(),
            progress: 0,
            priority: 0,
            status: "".to_string(),
            due: chrono::Local::now().date_naive()
        }
    }
}

// Enum used for sorting task lists
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TaskSort {
    #[default] None,
    Summary,
    Completed,
    Description,
    Progress,
    Priority,
    Status,
    Due
}

impl TaskSort {
    // Returns an array of values of the enum to iterate over
    pub fn iterator () -> Iter<'static, Self> {
        return [Self::None, Self::Summary, Self::Completed, Self::Description, Self::Progress, Self::Priority, Self::Status, Self::Due].iter()
    }
}

#[derive(Default)]
pub struct TaskList {
    pub name: String,
    pub tasks: Vec<Task>,
    pub color: egui::Color32
}

impl TaskList {
    // Sort a task list based on a TaskSort passed into the function
    pub fn sort (&mut self, sort_by: TaskSort) {
        match sort_by {
            TaskSort::None => {},
            TaskSort::Summary => self.tasks.sort_by(|a, b| { a.summary.to_lowercase().cmp(&b.summary.to_lowercase()) }),
            TaskSort::Completed => self.tasks.sort_by(|a, b| { a.completed.cmp(&b.completed) }),
            TaskSort::Description => self.tasks.sort_by(|a, b| { a.description.to_lowercase().cmp(&b.description.to_lowercase()) }),
            TaskSort::Progress => self.tasks.sort_by(|a, b| { a.progress.cmp(&b.progress) }),
            TaskSort::Priority => self.tasks.sort_by(|a, b| { a.priority.cmp(&b.priority) }),
            TaskSort::Status => self.tasks.sort_by(|a, b| { a.status.to_lowercase().cmp(&b.status.to_lowercase()) }),
            TaskSort::Due => self.tasks.sort_by(|a, b| { a.due.cmp(&b.due) }),
        }
    }
}
