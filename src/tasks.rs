use chrono::prelude::*;
use eframe::egui;

pub struct Task {
    summary: String,
    completed: bool,
    description: String,
    completion: Option<i8>,
    priority: Option<i8>,
    status: Option<String>,
    due: Option<NaiveDate>,
    colour: egui::Color32,
}
