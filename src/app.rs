use eframe::egui;
use crate::tasks::Task;
use crate::ui_elements;

#[derive(Default)]
pub struct App { // Stores application state
    input_task: Task,
}

impl App { // Defines the default application state
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// Define how the app behaves based on the app state
impl eframe::App for App {

// ┌ Main render loop function ───────────────────────────────────────┐ ┌ Main UI panel ─────────────────────────────┐
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { egui::CentralPanel::default().show(ctx, |ui| {

    ui.heading("Create a task");

    // Task name input
    ui.horizontal(|ui| {
        let name_label = ui.label("Task name ");
        ui.text_edit_singleline(&mut self.input_task.summary)
            .labelled_by(name_label.id);
    });

    // Task progress and priority sliders
    ui.horizontal(|ui| {
        let progress_label = ui.label("Progress ");
        ui_elements::percentage_slider(ui, &mut self.input_task.progress)
            .labelled_by(progress_label.id);

        let priority_label = ui.label("Priority ");
        ui_elements::percentage_slider(ui, &mut self.input_task.priority)
            .labelled_by(priority_label.id);
    });
    
    // Task description input
    let desc_label = ui.label("Task description ");
    ui.text_edit_multiline(&mut self.input_task.description)
        .labelled_by(desc_label.id);

    // Task status input
    ui.horizontal(|ui| {
        let status_label = ui.label("Task status ");
        ui.text_edit_singleline(&mut self.input_task.status)
            .labelled_by(status_label.id);
    });

    // Task complete checkbox
    ui.checkbox(&mut self.input_task.completed, "Task is complete");

});}

}
