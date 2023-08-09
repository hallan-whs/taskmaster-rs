// ----------------------------------------------------------------------------
// Modal window which shows a task's full details and allows the user to edit a task.
// ----------------------------------------------------------------------------

use eframe::egui;

use crate::task::Task;

use super::*;

pub fn spawn(task: &mut Task, ctx: &egui::Context) {
    egui::Window::new(format!("Edit task: {}", task.summary))
        .id(task.uuid.to_string().into())
        .open(&mut task.show_modal.clone().borrow_mut())
        .show(ctx, |ui| {
            // Set global ui scale
            ctx.set_pixels_per_point(1.75);

            // Set spacing between panels
            ui.spacing_mut().item_spacing = egui::Vec2::splat(7.5);

            crate::ui_elements::basic_frame().show(ui, |ui| {
                // Task editing UI
                task_edit::full(ui, task);
            })
        });
}
