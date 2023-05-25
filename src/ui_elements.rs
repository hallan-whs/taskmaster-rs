//-------------------------------------------------------------------------------------------------------------------------
// Collection of custom user interface elements that are commonly repeated throughout the app
// Or which were too big to put somewhere else
//-------------------------------------------------------------------------------------------------------------------------

use eframe::egui;
use egui::{Ui, Response};

// Custom percentage slider - this is needed surprisingly often
pub fn percentage_slider (ui: &mut Ui, percent: &mut u8) -> Response {
    ui.add(egui::Slider::new(percent, 0..=100)
        .show_value(true).custom_formatter(|n, _| {
            n.to_string() + "%"
        })
    )
}
