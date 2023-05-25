mod tasks;
use eframe::egui;

const WINDOW_TITLE: &str = "Hello world";

fn main() {
    // Initialize the window with a default option set and the app defined below
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(WINDOW_TITLE, native_options, Box::new(|cc| Box::new(App::new(cc)))).unwrap();
}

// Basic hello world app
#[derive(Default)]
struct App { // Stores application state

}

impl App { // Defines the default application state
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}
