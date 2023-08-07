// ----------------------------------------------------------------------------
// Provides UI for editing any task passed into it.
// There are two versions, a "lite" minimal version, and a "full" version
// The full version contains a field for every field of a Task struct
// The lite version only has a field for task summary and completion
// ----------------------------------------------------------------------------

use convert_case::{Case, Casing};
use eframe::egui::{self, Ui};
use egui_extras::DatePickerButton;

use crate::task::TaskStatus;

use super::percentage_slider;

pub fn lite(ui: &mut Ui, task: &mut crate::task::Task) {
    ui.horizontal(|ui| {
        // Task name input
        let name_label = ui.label("Task name");
        ui.text_edit_singleline(&mut task.summary)
            .labelled_by(name_label.id);

        // Task completion checkbox
        ui.checkbox(&mut task.completed, "");
    });
}

pub fn full(ui: &mut Ui, task: &mut crate::task::Task) {
    // Task name input
    ui.horizontal(|ui| {
        let name_label = ui.label("Task name");
        ui.text_edit_singleline(&mut task.summary)
            .labelled_by(name_label.id);
    });

    // Task progress and priority sliders
    ui.horizontal(|ui| {
        let progress_label = ui.label("Progress");
        percentage_slider(ui, &mut task.progress).labelled_by(progress_label.id);

        let priority_label = ui.label("Priority");
        ui.add(egui::Slider::new(&mut task.priority, 0..=10))
            .labelled_by(priority_label.id);
    });

    // Task description input
    let desc_label = ui.label("Task description");
    ui.text_edit_multiline(&mut task.description)
        .labelled_by(desc_label.id);

    // Task status input
    egui::ComboBox::from_label("Status")
        .selected_text(format!("{:?}", &task.status).to_case(Case::Title)) // Show selected status
        .show_ui(ui, |ui| {
            for _status in TaskStatus::iterator() {
                // Iterate over possible statuses and show each as an option
                ui.selectable_value(
                    &mut task.status,
                    *_status,
                    format!("{:?}", _status).to_case(Case::Title),
                );
            }
        });

    ui.horizontal(|ui| {
        let mut has_due = task.due.is_some();

        ui.checkbox(&mut has_due, "Task has due date");

        if has_due & task.due.is_some() {
            let mut due = task.due.unwrap();

            // Due date input
            ui.add(DatePickerButton::new(&mut due));

            task.due = Some(due);
        } else if has_due & task.due.is_none() {
            task.due = Some(chrono::Local::now().date_naive())
        } else {
            task.due = None;
        }
    });

    // Task complete checkbox
    ui.checkbox(&mut task.completed, "Task is complete");

    if task.status == TaskStatus::Completed { task.completed = true }
}
