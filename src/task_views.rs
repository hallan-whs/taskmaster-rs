// ----------------------------------------------------------------------------
// Implements the task view system. Every task view defines a function which
// takes a task list and determines how it should be displayed.
// This allows the program to take any type which defines that function and
// use it to display the task list.
// ----------------------------------------------------------------------------

use convert_case::Case;
use convert_case::Casing;
use eframe::egui;
use egui::{RichText, Ui};

use crate::task::*;
use crate::ui_elements;

/// Trait that every task view must implement
/// This ensures that every task view includes a function for displaying itself,
/// as well as allowing the program to ask for any task view and display it
pub trait TaskView {
    /// Takes a UI, a task list, and some extra parameters,and uses them to
    /// display the list in whatever view is implementing the function
    fn display(ui: &mut Ui, task_list: &mut TaskList, show_completed_tasks: bool);
}

pub struct ClassicView;

impl TaskView for ClassicView {
    fn display(ui: &mut Ui, task_list: &mut TaskList, show_completed_tasks: bool) {
        let mut last_task_uuid = 0u128;
        if let Some(lasttask) = task_list.tasks.last() {
            last_task_uuid = lasttask.uuid.as_u128();
        }

        ui.separator();

        // Iterates over each task in the task list, keeping or removing each task
        // based on a returned boolean which is determined within each iteration
        // In this case, the returned boolean indicates whether the task's
        // remove button has been pressed. This allows for removal of an element
        // from the vector of tasks without getting an IndexOutOfRange error
        // when the iterator gets to a place where a removed task used to be,
        // which is what would happen with something like a for loop.
        task_list.tasks.retain_mut(|task| {
            // This is the boolean which determines whether a task is removed from the vector
            let mut keep = true;

            // If show_completed_tasks is enabled, completed tasks will be shown, otherwise they will be hidden
            if show_completed_tasks || !task.completed {
                ui.horizontal(|ui| {
                    // Expand to fit window
                    ui.set_width(ui.available_width());

                    // Create rich text containing the task's summary
                    let mut task_text = RichText::new(&task.summary);
                    if task.completed {
                        task_text = task_text.strikethrough();
                    }

                    // Create rich text containing the task's description
                    let mut desc_text = task.description.replace('\n', " ");
                    desc_text.truncate(20);
                    let desc_text = desc_text.trim().to_string() + "...";
                    let mut desc_text = RichText::new(desc_text);
                    if task.completed {
                        desc_text = desc_text.strikethrough();
                    }

                    if !task.description.is_empty() {
                        // Show task description
                        ui.label(desc_text);
                    }

                    // Create a checkbox with previously created text
                    ui.checkbox(&mut task.completed, task_text);

                    // Right-aligned, right-to-left UI segment
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Click this to show a modal with a task's full details
                        // Doesn't spawn it if there's already one present
                        if ui.button("...").clicked() {
                            *task.show_modal.borrow_mut() = true;
                        };

                        // If the button is clicked, mark task for removal
                        keep = !ui.button("✖").clicked();

                        // If the task has a due date, display it
                        if let Some(mut due) = task.due {
                            ui.add(egui_extras::DatePickerButton::new(&mut due));
                        }

                        // If the task's priority isn't zero, display it
                        if task.priority != 0 {
                            ui.add(egui::Slider::new(&mut task.priority, 0..=10));
                        }

                        // If the task's priority isn't zero, display it
                        if task.progress != 0 {
                            ui_elements::percentage_slider(ui, &mut task.progress);
                        }

                        // Create dropdown containing the task's status
                        if task.status != TaskStatus::InProgress {
                            // Combo box IDs are normally generated using their label.
                            // If two elements have the same label, weird stuff happens
                            // So an id is generated using the `uuid` library.
                            egui::ComboBox::new(task.uuid.to_u128_le(), "Status")
                                .selected_text(format!("{:?}", &task.status).to_case(Case::Title)) // Show selected status
                                .show_ui(ui, |ui| {
                                    for status in TaskStatus::iterator() {
                                        // Iterate over possible statuses and show each as an option
                                        ui.selectable_value(
                                            &mut task.status,
                                            *status,
                                            format!("{:?}", status).to_case(Case::Title),
                                        );
                                    }
                                });
                        }
                    });
                });
                // If the currently addressed task is not the final task in the list,
                // Draw a separator between the tasks
                if task.uuid.as_u128() != last_task_uuid {
                    ui.separator();
                }

                if task.status == TaskStatus::Completed {
                    task.completed = true
                }
            }

            // Spawn a modal if told to
            if *task.show_modal.borrow() {
                ui_elements::task_modal::spawn(task, ui.ctx());
            }

            keep // If this is false, task is removed from the vector
        });
    }
}
