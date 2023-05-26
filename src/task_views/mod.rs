use eframe::egui::Ui;
use crate::tasks::*;

pub trait TaskView { 
    // Trait that every task view must implement
    // This ensures that every task view includes a function for displaying itself,
    // as well as allowing the program to ask for any task view and display it

    fn display ( 
        // Takes a UI, a task list, and some extra parameters,
        // and uses them to display the list in whatever view is implementing the function
        ui: &mut Ui, 
        task_list: TaskList,
        show_completed_tasks: bool,
    );
}
