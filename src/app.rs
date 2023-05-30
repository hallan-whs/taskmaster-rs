use eframe::egui;
use egui_extras::DatePickerButton;

use crate::tasks::*;
use crate::task_views::*;
use crate::ui_elements;

#[derive(Default)]
pub struct App { // Stores application state
    input_task: Task,
    input_task_list: TaskList,
    show_completed_tasks: bool,
    sort_by: TaskSort,
}

impl App { // Defines the default application state
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            show_completed_tasks: false,
            ..Self::default() // Sets everything to default except for the variables defined above
        }
    }
}

// Define how the app behaves based on the app state
impl eframe::App for App {

// ┌ Main render loop function ───────────────────────────────────────┐ ┌ Main UI panel ─────────────────────────────┐
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) { egui::CentralPanel::default().show(ctx, |ui| {

    // Set global ui scale
    ctx.set_pixels_per_point(1.5);

    // Set spacing between panels
    ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);


    //Task input panel
    ui_elements::basic_frame().show(ui, |ui| {

        // Expand to fit window
        ui.set_width(ui.available_width());
        
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

        // Due date input
        ui.add(DatePickerButton::new(&mut self.input_task.due));

        // Task complete checkbox
        ui.checkbox(&mut self.input_task.completed, "Task is complete");

        if ui.button("Add task").clicked() {

            // Add specified task to the list of tasks
            self.input_task_list.tasks.push(self.input_task.clone());

            // Enable showing completed tasks if the task that was just added is marked as complete
            // This prevents confusion from a newly added task not being shown if it's complete
            if self.input_task.completed & !self.show_completed_tasks {
                self.show_completed_tasks = true;
            }

        }

    });


    //Task list panel
    ui_elements::basic_frame().show(ui, |ui| {
        
        // Expand to fit window
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());
        
        // Top bar, with sorting and other list options
        ui.horizontal(|ui| {

            // Checkbox to show tasks that have been completed
            ui.checkbox(&mut self.show_completed_tasks, "Show completed tasks");

            ui.label("| ");

            // Button to sort task list by chosen field
            if ui.button("Sort").clicked() {
                self.input_task_list.sort(self.sort_by);
            }

            // Dropdown to choose which field to sort by
            ui.label("by");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", &self.sort_by)) // Show selected sort field
                .show_ui(ui, |ui| {
                    for _sort_by in TaskSort::iterator() { // Iterate over sortable fields and display each as an option
                        ui.selectable_value(&mut self.sort_by, *_sort_by, format!("{:?}", _sort_by));
                    }
                });

        });

        // Scrollable area
        egui::ScrollArea::vertical().show_rows(ui, 14.0, self.input_task_list.tasks.len(), |ui, _| {

            // Display tasks in classic view
            ClassicView::default().display(ui, &mut self.input_task_list, self.show_completed_tasks);

        });

    });

});}

}
