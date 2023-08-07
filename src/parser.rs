// ----------------------------------------------------------------------------
// Functionality for parsing a TaskList to or from an iCal file with VTODOs.
// ----------------------------------------------------------------------------

use std::{fs::File, io::BufReader, path::Path};

use eframe::egui;

use crate::task::*;

impl TaskList {
    /// Converts an iCal file to a TaskList.
    ///
    /// ```
    /// use taskmaster_rs::task::TaskList;
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// let a = TaskList::from_file(Path::new("test.ics"));
    /// let afmt = format!("{:?}", a);
    ///
    /// assert_eq!(afmt, "Ok(TaskList { name: \"T_tmtest\", tasks: [Task { summary: \"Task 1\", completed: false, description: \"description\\n\", progress: 47, priority: 9, status: NeedsAction, due: None, show_modal: false }], color: Color32([83, 130, 163, 255]) })");
    /// ```
    pub fn from_file(path: &Path) -> Result<TaskList, ParseFromFileError> {
        let file = BufReader::new(File::open(path).unwrap());

        let reader = ical::PropertyParser::from_reader(file);

        let mut list: TaskList = TaskList::default();
        let mut tasks: Vec<Task> = vec![];
        let mut task_counter = 0usize;

        // Iterate through each line parsed from the file
        for line in reader {
            // If the line is valid
            if let Ok(property) = line {
                // Checks what the line is saying
                match property.name.as_str() {
                    // Set calendar name
                    "X-WR-CALNAME" => list.name = property.value.unwrap(),
                    // Set calendar color
                    "X-APPLE-CALENDAR-COLOR" => {
                        if let Ok(rgb) = hex_rgb::convert_hexcode_to_rgb(property.value.unwrap()) {
                            list.color = egui::Color32::from_rgb(rgb.red, rgb.green, rgb.blue);                       
                        }
                    },
                    // Checks for a BEGIN statement
                    "BEGIN" => {
                        match property.value.unwrap().as_str() {
                            // If it's starting a new task, add a task to the vector of tasks
                            "VTODO" => {
                                tasks.push(Task::default())
                            },
                            // If it's just starting the file, do nothing
                            "VCALENDAR" => (),
                            // If it's starting anything else, return an error
                            _ => return Err(ParseFromFileError::NonTaskItem),
                        }
                    }
                    // Set the currently addressed task's summary
                    "SUMMARY" => {
                        tasks[task_counter].summary = property.value.unwrap();
                    }
                    // Set the currently addressed task's priority
                    "PRIORITY" => {
                        tasks[task_counter].priority = property.value.unwrap().parse().unwrap();
                    }
                    // Set the currently addressed task's completion percent
                    "PERCENT-COMPLETE" => {
                        tasks[task_counter].progress = property.value.unwrap().parse().unwrap();
                    }
                    // Set the currently addressed task's status
                    "STATUS" => {
                        let task = &mut tasks[task_counter];
                        task.status = match property.value.unwrap().as_str() {
                            "IN-PROGRESS" => TaskStatus::InProgress,
                            "NEEDS-ACTION" => TaskStatus::NeedsAction,
                            "COMPLETED" => TaskStatus::Completed,
                            "CANCELLED" => TaskStatus::Cancelled,
                            _ => return Err(ParseFromFileError::InvalidStatus),
                        }
                    }
                    // Set the currently addressed task's description
                    "DESCRIPTION" => {
                        tasks[task_counter].description = property.value.unwrap()
                            .replace("\\n", "\n");
                    }
                    // If the file says that the task description is complete,
                    // iterate the task counter so that it can be used to address
                    // the task that is added next (if any).
                    "END" => {
                        if Some("VTODO".to_string()) == property.value {
                            task_counter += 1;
                        }
                    }
                    // If the line isn't any of the above, just do nothing
                    _ => (),
                }
            } else {
                // If the line is invalid, return an error
                return Err(ParseFromFileError::InvalidFile);
            }
        }
        list.tasks = tasks;
        // Everything is all good, so return the list
        Ok(list)
    }
}

// Possible errors for parsing from a file
#[derive(Debug)]
pub enum ParseFromFileError {
    InvalidFile,
    InvalidStatus,
    NonTaskItem,
}
