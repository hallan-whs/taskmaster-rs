use std::path::Path;

use taskmaster_rs::{app, task::TaskList};

const WINDOW_TITLE: &str = "Hello world";

fn main() {
    TaskList::from_file(Path::new("test.ics")).unwrap().to_file(Path::new("test2.ics")).unwrap();

    // Initialize the window with a default option set and run the app defined in app.rs
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        WINDOW_TITLE,
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
    .unwrap();

}
