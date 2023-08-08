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
        ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

        //Task input panel
        basic_frame().show(ui, |ui| {
            // Expand to fit window
            ui.set_height(ui.available_height());
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    // Task editing UI
                    task_edit::full(ui, task);
                });
            })
        })
    });
}
