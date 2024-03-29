//-----------------------------------------------------------------------------
// Collection of custom user interface elements that are commonly used in the app
// Or which were too big to put somewhere else.
//-----------------------------------------------------------------------------

pub mod task_edit;
pub mod task_modal;

use eframe::egui;
use egui::{Response, Ui};

// Custom percentage slider
pub fn percentage_slider(ui: &mut Ui, percent: &mut u8) -> Response {
    ui.add(
        egui::Slider::new(percent, 0..=100)
            .show_value(true)
            .custom_formatter(|n, _| n.to_string() + "%"),
    )
}

// Commonly used frame for different panels of the application
pub fn basic_frame() -> egui::Frame {
    egui::Frame::default()
        .inner_margin(10.)
        .stroke(eframe::epaint::Stroke {
            width: 1.,
            color: egui::Color32::GRAY,
        })
}
