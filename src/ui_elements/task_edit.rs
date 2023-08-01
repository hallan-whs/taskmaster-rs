// ----------------------------------------------------------------------------
// Provides UI for editing any task passed into it.
// There are two versions, a "lite" minimal version, and a "full" version
// The full version contains a field for every field of a Task struct
// The lite version only has a field for task summary and completion
// ----------------------------------------------------------------------------

use eframe::egui::Ui;
use egui_extras::DatePickerButton;

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
        percentage_slider(ui, &mut task.progress)
            .labelled_by(progress_label.id);

        let priority_label = ui.label("Priority");
        percentage_slider(ui, &mut task.priority)
            .labelled_by(priority_label.id);
    });

    // Task description input
    let desc_label = ui.label("Task description");
    ui.text_edit_multiline(&mut task.description)
        .labelled_by(desc_label.id);

    // Task status input
    ui.horizontal(|ui| {
        let status_label = ui.label("Task status");
        ui.text_edit_singleline(&mut task.status)
            .labelled_by(status_label.id);
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
}
