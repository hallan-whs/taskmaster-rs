// ----------------------------------------------------------------------------
// Determines the app's state and behavior.
// An instance of the app is instantiated in the main function,
// and runs the update loop as it is implemented below.
// ----------------------------------------------------------------------------

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use eframe::egui;
use egui_file::FileDialog;

use crate::parser::ParseFromFileError;
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
    // Stores the file dialog to choose which task list file to import
    import_dialog: Option<FileDialog>,
    // Stores the list parsed from the imported file
    imported_list: Option<Result<TaskList, ParseFromFileError>>,
    // Stores the file dialog to choose where to export the task list to a file
    export_dialog: Option<FileDialog>,
}

impl App {
    // Defines the default application state
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            show_completed_tasks: false,
            ..Default::default() // Everything else is default
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
            ui.spacing_mut().item_spacing = egui::Vec2::splat(7.5);

            // Task list import/export panel
            ui_elements::basic_frame().show(ui, |ui| {
                ui.set_width(ui.available_width());

                // Import a task list file
                ui.horizontal(|ui| {
                    // This stores the path of the imported file
                    let mut opened_file: Option<PathBuf> = None;

                    if (ui.button("Open file")).clicked() {
                        // Create a file dialog, which reads from and writes to 
                        // the file path variable created earlier
                        let mut dialog = FileDialog::open_file(opened_file.clone());
                        // Mark it as opened
                        dialog.open();
                        // This makes sure that the import and export dialogs 
                        // aren't open at the same time
                        self.export_dialog = None;
                        // Stores the dialog in the app state
                        // This means it can be used in later update loops
                        self.import_dialog = Some(dialog);
                    }

                    // If there is an import dialog stored in the app's state
                    if let Some(dialog) = &mut self.import_dialog {
                        // If a file has been picked using the dialog
                        if dialog.show(ctx).selected() {
                            // If the file has a valid path
                            if let Some(file) = dialog.path() {
                                // Store the path
                                opened_file = Some(file.to_path_buf())
                            }
                        }
                    }

                    // If there is a file path being stored
                    if let Some(file) = &mut opened_file {
                        // Try to get the calendar data from the file at that path
                        let parse_result = TaskList::from_ical_file(file);
                        // Store the result in the app state
                        self.imported_list = Some(parse_result);
                    }

                    // If there is a task list parsing result stored 
                    if let Some(list) = &self.imported_list {
                        match list {
                            // If the parse was successful
                            Ok(list) => {
                                // Give information about the task list in the file
                                ui.label(format!("File contains list '{}'", &list.name));
                                // Display a button to import the task list from the file into the app
                                if ui.button("Import ( warning: overwrites current list )").clicked() {
                                    self.input_task_list = list.clone();
                                }
                            }
                            // If the parse was unsuccessful
                            Err(e) => {
                                // Create an appropriate string from all the possible errors
                                let err_str = match e {
                                    ParseFromFileError::InvalidFile => "Invalid task list file",
                                    ParseFromFileError::NonTaskItem => "File contained items that were not todo items. Was it exported from calendar software?",
                                    ParseFromFileError::InvalidField => "File contains invalid data",
                                };
                                // Display a label showing the error
                                ui.label(err_str);
                            }
                        }
                    }
                });

                // Export task list to a file
                ui.horizontal(|ui| {

                    if ui.button("Export task list").clicked() {
                        // Create a file export dialog
                        let mut dialog = FileDialog::save_file(None);
                        // Mark it as opened
                        dialog.open();
                        // This makes sure that the import and export dialogs 
                        // aren't open at the same time
                        self.import_dialog = None;
                        // Stores the dialog in the app state
                        // This means it can be used in later update loops
                        self.export_dialog = Some(dialog);
                    }

                    // If there is an import dialog stored in the app's state
                    if let Some(dialog) = &mut self.export_dialog {
                        // If a file has been picked using the dialog
                        if dialog.show(ctx).selected() {
                            // If the path is valid
                            if let Some(file) = dialog.path() {
                                // Get contents of file which will be exported
                                let list_str = self.input_task_list.to_ical_string();
                                // Create the file which will have the data
                                let mut f = File::create(file).unwrap();
                                // Write the data to the file
                                f.write(list_str.as_bytes()).unwrap();
                            }
                        }
                    }

                    // This lets you change the list's name before exporting it
                    ui.label("List name:");
                    ui.text_edit_singleline(&mut self.input_task_list.name);
                });
            });

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
                                self.input_task.uuid = uuid::Uuid::new_v4();

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
                        let btn_str = if self.show_full_edit {
                            "⏶"
                        } else {
                            "⏷"
                        };
                        if ui.button(btn_str).clicked() {
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

                // Scrollable area that shows all the tasks
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    14.,
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
