// ----------------------------------------------------------------------------
// Defines the data structures which hold tasks and task lists.
// ----------------------------------------------------------------------------

use chrono::prelude::*;
use eframe::egui;
use std::{cmp::Ordering, slice::Iter};

// Holds the data for a task
#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub summary: String,
    pub completed: bool,
    pub description: String,
    pub progress: u8,
    pub priority: u8,
    pub status: TaskStatus,
    pub due: Option<NaiveDate>,
    pub created: NaiveDateTime,
    pub show_modal: bool,
}

// Define default task
impl Default for Task {
    fn default() -> Self {
        Self {
            summary: "New task".to_string(),
            completed: false,
            description: "".to_string(),
            progress: 0,
            priority: 0,
            status: TaskStatus::InProgress,
            due: None,
            created: chrono::Utc::now().naive_local(),
            show_modal: false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TaskList {
    pub name: String,
    pub tasks: Vec<Task>,
    pub color: egui::Color32,
}

impl Default for TaskList {
    fn default() -> Self {
        TaskList {
            name: "New list".to_string(),
            tasks: vec![],
            color: egui::Color32::DEBUG_COLOR,
        }
    }
}

impl TaskList {
    // Sort a task list based on a TaskSort passed into the function.
    #[rustfmt::skip]
    pub fn sort(&mut self, sort_by: TaskSort) {
        match sort_by {
            TaskSort::None => {}
            TaskSort::Summary => self.tasks.sort_by(|a, b| a.summary.to_lowercase().cmp(&b.summary.to_lowercase())),
            TaskSort::Completed => self.tasks.sort_by(|a, b| a.completed.cmp(&b.completed)),
            TaskSort::Description => self.tasks.sort_by(|a, b| { a.description.to_lowercase().cmp(&b.description.to_lowercase()) }),
            TaskSort::Progress => self.tasks.sort_by(|a, b| a.progress.cmp(&b.progress)),
            TaskSort::Priority => self.tasks.sort_by(|a, b| a.priority.cmp(&b.priority)),
            TaskSort::Status => self.tasks.sort_by(|a, b| a.status.partial_cmp(&b.status).unwrap()),
            // This makes sure that tasks with due dates show up before ones without
            // As well as making sure that the sooner the date, the higher up the task
            TaskSort::Due => {
                self.tasks.sort_by(|a, b| {
                    if let Some(a_due) = a.due {
                        if let Some(b_due) = b.due {
                            a_due.cmp(&b_due)
                        } else {
                            Ordering::Less
                        }
                    } else {
                        Ordering::Greater
                    }
                });
            },
        }
    }

    // Adds a task to a task list, and executes any other required code
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // Returns whether or not the task list has any modals which are already open
    pub fn has_any_modals(&self) -> bool {
        let mut has_any_modals = false;
        for task in self.tasks.iter() {
            if task.show_modal {
                has_any_modals = true;
                break;
            }
        }
        has_any_modals
    }
}

// The STATUS field of a VTODO can only have certain values.
// This enum is used to choose between the valid values of this field.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TaskStatus {
    #[default]
    InProgress,
    NeedsAction,
    Completed,
    Cancelled,
}

impl TaskStatus {
    // Returns an array of values of the enum, for other code to iterate over
    pub fn iterator() -> Iter<'static, Self> {
        return [
            Self::InProgress,
            Self::NeedsAction,
            Self::Completed,
            Self::Cancelled,
        ]
        .iter();
    }
}

// Enum used for sorting task lists
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TaskSort {
    #[default]
    None,
    Summary,
    Completed,
    Description,
    Progress,
    Priority,
    Status,
    Due,
}

impl TaskSort {
    // Returns an array of values of the enum, for other code to iterate over
    pub fn iterator() -> Iter<'static, Self> {
        return [
            Self::None,
            Self::Summary,
            Self::Completed,
            Self::Description,
            Self::Progress,
            Self::Priority,
            Self::Status,
            Self::Due,
        ]
        .iter();
    }
}
