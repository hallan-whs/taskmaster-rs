use taskmaster_rs::app;

const WINDOW_TITLE: &str = "Taskmaster";

fn main() {
    // Initialize the window with a default option set and run the app defined in app.rs
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
    .expect("failed to start egui");
}
