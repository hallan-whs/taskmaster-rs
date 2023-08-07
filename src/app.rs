// ----------------------------------------------------------------------------
// Determines the app's state and behavior.
// An instance of the app is instantiated in the main function,
// and runs the update loop as it is implemented below.
// ----------------------------------------------------------------------------

use std::path::Path;

use eframe::egui;

use crate::task::*;
use crate::task_views::*;
use crate::ui_elements;

#[derive(Default)]
pub struct App {
    // Stores application state
    input_task: Task,
    input_task_list: TaskList,
    show_completed_tasks: bool,
    sort_by: TaskSort,
    show_full_edit: bool,
}

impl App {
    // Defines the default application state
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            show_completed_tasks: false,
            input_task_list: TaskList::from_file(Path::new("test.ics")).unwrap(), 
            ..Self::default() // Everything else is default
        }
    }
}

// Define how the app behaves based on the app state
impl eframe::App for App {
    // - Main render loop function ----------------------------------------
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // - Main UI panel ---------------------------
        egui::CentralPanel::default().show(ctx, |ui| {
            // Set global ui scale
            ctx.set_pixels_per_point(1.75);

            // Set spacing between panels
            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

            //Task input panel
            ui_elements::basic_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Expand to fit window
                    ui.set_width(ui.available_width());

                    // Decides whether to show the simple task editing UI
                    // or a simplified/minimal version
                    if self.show_full_edit {
                        // Full task editing UI
                        ui.vertical(|ui| {
                            ui_elements::task_edit::full(ui, &mut self.input_task);

                            if ui.button("Add task").clicked() {
                                // Add input task to the list of tasks
                                self.input_task_list.tasks.push(self.input_task.clone());

                                // Enable showing completed tasks if the task
                                // that was just added is marked as complete
                                // This prevents confusion from a newly added
                                // task not being shown if it's already marked
                                // as complete when it's added to the list.
                                if self.input_task.completed & !self.show_completed_tasks {
                                    self.show_completed_tasks = true;
                                }
                            }
                        });
                    } else {
                        // Simplified task editing UI
                        ui.horizontal(|ui| {
                            if ui.button("+").clicked() {
                                // Add input task to the list of tasks
                                self.input_task_list.add(Task {
                                    summary: self.input_task.clone().summary,
                                    completed: self.input_task.clone().completed,
                                    ..Task::default()
                                });

                                // Enable showing completed tasks if the task
                                // that was just added is marked as complete
                                // This prevents confusion from a newly added
                                // task not being shown if it's marked complete
                                if self.input_task.completed & !self.show_completed_tasks {
                                    self.show_completed_tasks = true;
                                }
                            }

                            ui_elements::task_edit::lite(ui, &mut self.input_task);
                        });
                    }

                    // This is the button which switches between the views
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        if ui.button("...").clicked() {
                            self.show_full_edit = !self.show_full_edit
                        }
                    });
                })
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
                            for _sort_by in TaskSort::iterator() {
                                // Iterate over sortable fields and display each as an option
                                ui.selectable_value(
                                    &mut self.sort_by,
                                    *_sort_by,
                                    format!("{:?}", _sort_by),
                                );
                            }
                        });
                });

                // Scrollable area
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    14.0,
                    self.input_task_list.tasks.len(),
                    |ui, _| {
                        // Display tasks in classic view
                        ClassicView::display(
                            ui,
                            &mut self.input_task_list,
                            self.show_completed_tasks,
                        );
                    },
                );
            });
        });
    }
}
