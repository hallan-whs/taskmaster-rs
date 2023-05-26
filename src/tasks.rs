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
    pub due: NaiveDate,
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
            due: NaiveDate::default(),
        }
    }
}

#[derive(Default)]
pub struct TaskList {
    pub name: String,
    pub tasks: Vec<Task>,
    pub color: egui::Color32
}
