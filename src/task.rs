// ----------------------------------------------------------------------------
// Defines the data structures which hold tasks and task lists.
// ----------------------------------------------------------------------------

use chrono::prelude::*;
use eframe::egui;
use std::{cell::RefCell, cmp::Ordering, rc::Rc, slice::Iter};

// Holds the data for a task
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub uuid: uuid::Uuid,
    pub summary: String,
    pub completed: bool,
    pub description: String,
    pub progress: u8,
    pub priority: u8,
    pub status: Status,
    pub due: Option<NaiveDate>,
    pub created: NaiveDateTime,
    pub show_modal: Rc<RefCell<bool>>,
}

// Define default task
impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            summary: String::from("New task"),
            completed: false,
            description: String::new(),
            progress: 0,
            priority: 0,
            status: Status::InProgress,
            due: None,
            created: chrono::Utc::now().naive_local(),
            show_modal: Rc::new(RefCell::new(false)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct TaskList {
    pub name: String,
    pub tasks: Vec<Task>,
    pub color: egui::Color32,
}

impl Default for TaskList {
    fn default() -> Self {
        Self {
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
            TaskSort::Status => self.tasks.sort_by(|a, b| a.status.partial_cmp(&b.status).expect("could not compare summaries")),
            // This makes sure that tasks with due dates show up before ones without
            // As well as making sure that the sooner the date, the higher up the task
            TaskSort::Due => {
                self.tasks.sort_by(|a, b| {
                    a.due.map_or(Ordering::Equal, |a_due| {
                        b.due.map_or(Ordering::Greater, |b_due| {
                            a_due.cmp(&b_due)
                        })
                    })
                });
            },
        }
    }

    // Adds a task to a task list, and executes any other required code
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
}

// The STATUS field of a VTODO can only have certain values.
// This enum is used to choose between the valid values of this field.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Status {
    NeedsAction,
    #[default]
    InProgress,
    Completed,
    Cancelled,
}

impl Status {
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
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
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
