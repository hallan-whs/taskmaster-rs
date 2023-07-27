// ----------------------------------------------------------------------------
// Implements the task view system. Every task view defines a function which
// takes a task list and determines how it should be displayed.
// This allows the program to take any type which defines that function and
// use it to display the task list
// ----------------------------------------------------------------------------

use eframe::egui;
use egui::{RichText, Ui};

use crate::tasks::*;
use crate::ui_elements;

/// Trait that every task view must implement
/// This ensures that every task view includes a function for displaying itself,
/// as well as allowing the program to ask for any task view and display it
pub trait TaskView {
    /// Takes a UI, a task list, and some extra parameters,and uses them to
    /// display the list in whatever view is implementing the function
    fn display(&self, ui: &mut Ui, task_list: &mut TaskList, show_completed_tasks: bool);
}

pub struct ClassicView;

impl TaskView for ClassicView {
    fn display(&self, ui: &mut Ui, task_list: &mut TaskList, show_completed_tasks: bool) {
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

                    ui.checkbox(&mut task.completed, task_text);

                    // Contains right-aligned elements, right-to-left
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                        keep = !ui.button("✖").clicked(); // If the button is clicked, mark task for removal
                        ui_elements::percentage_slider(ui, &mut task.progress);
                    });
                });
            }

            keep // If this is false, task is removed from the vector
        });
    }
}
