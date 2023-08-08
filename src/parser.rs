// ----------------------------------------------------------------------------
// Functionality for parsing a TaskList to or from an iCal file with VTODOs.
// ----------------------------------------------------------------------------

use std::{
    fs::File, 
    io::{BufReader, Write},
    path::Path
};

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
    /// assert_eq!(afmt, 
    /// "Ok(TaskList { name: \"T_tmtest\", tasks: [Task { summary: \"Task 1\", \
    /// completed: false, description: \"description\\n\", progress: 47, priority: 9, \
    /// status: NeedsAction, due: None, show_modal: false }], color: Color32([83, 130, 163, 255]) })"
    /// );
    /// ```
    pub fn from_file(path: &Path) -> Result<TaskList, ParseFromFileError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(ParseFromFileError::InvalidFile),
        };

        let reader = BufReader::new(file);

        let lines = ical::PropertyParser::from_reader(reader);

        let mut list: TaskList = TaskList::default();
        let mut tasks: Vec<Task> = vec![];
        let mut task_counter = 0usize;

        // Iterate through each line parsed from the file
        for line in lines {
            // If the line is valid
            if let Ok(property) = line {
                // Makes sure the line is saying something
                let value;
                if let Some(val) = property.value {
                    value = val.to_string();
                } else {
                    return Err(ParseFromFileError::InvalidField);
                }
                // Checks what the line is saying
                match property.name.as_str() {
                    // Set calendar name
                    "X-WR-CALNAME" => list.name = value,
                    // Set calendar color
                    "X-APPLE-CALENDAR-COLOR" => {
                        if let Ok(rgb) = hex_rgb::convert_hexcode_to_rgb(value) {
                            list.color = egui::Color32::from_rgb(rgb.red, rgb.green, rgb.blue);                       
                        }
                    },
                    // Checks for a BEGIN statement
                    "BEGIN" => {
                        match value.as_str() {
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
                        tasks[task_counter].summary = value;
                    }
                    // Set the currently addressed task's due date
                    "DUE" => {
                        let datestr = value;
                        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(&datestr, "%Y%m%dT%H%M%S") {
                            tasks[task_counter].due = Some(date.date());
                        } else {
                            return Err(ParseFromFileError::InvalidField)
                        }
                    }
                    // Set the currently addressed task's priority
                    "PRIORITY" => {
                        if let Ok(priority) = value.parse() {
                            tasks[task_counter].priority = priority;
                        } else {
                            return Err(ParseFromFileError::InvalidField);
                        }
                    }
                    // Set the currently addressed task's completion percent
                    "PERCENT-COMPLETE" => {
                        if let Ok(progress) = value.parse() {
                            tasks[task_counter].progress = progress;
                        } else {
                            return Err(ParseFromFileError::InvalidField);
                        }
                    }
                    // Set the currently addressed task's status
                    "STATUS" => {
                        let task = &mut tasks[task_counter];
                        task.status = match value.as_str() {
                            "IN-PROGRESS" => TaskStatus::InProgress,
                            "NEEDS-ACTION" => TaskStatus::NeedsAction,
                            "COMPLETED" => {
                                task.completed = true;
                                TaskStatus::Completed
                            },
                            "CANCELLED" => TaskStatus::Cancelled,
                            _ => return Err(ParseFromFileError::InvalidStatus),
                        }
                    }
                    // Set the currently addressed task's description
                    "DESCRIPTION" => {
                        tasks[task_counter].description = value
                            .replace("\\n", "\n");
                    }
                    // If the file says that the task description is complete,
                    // iterate the task counter so that it can be used to address
                    // the task that is added next (if any).
                    "END" => {
                        if value == "VTODO".to_string() {
                            task_counter += 1;
                        }
                    }
                    // Store the task's creation date
                    "CREATED" => {
                        let datestr = value;
                        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(&datestr, "%Y%m%dT%H%M%S") {
                            tasks[task_counter].created = date;
                        } else {
                            return Err(ParseFromFileError::InvalidField)
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

    pub fn to_file(&self, path: &Path) -> std::io::Result<File> {

        // Initiate text that will eventually be added to the calendar file
        // As well as adding some initial variables 
        // {:X} in a format string changes decimal numbers to hexadecimal
        let mut ical_text = format!("\
BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//ical-rs//github.com//
X-WR-CALNAME:{}
X-APPLE-CALENDAR-COLOR:{}
REFRESH-INTERVAL;VALUE=DURATION:PT4H
X-PUBLISHED-TTL:PT4H \
        ", self.name, format!("#{:X}{:X}{:X}", self.color.r(), self.color.g(), self.color.b()));

        // Add data for every todo item
        for task in &self.tasks {
            // Begins the task data
            ical_text.push_str("\nBEGIN:VTODO\n");
            // Generates a unique UID for the task
            ical_text.push_str(format!("UID:{}\n", uuid::Uuid::new_v4().to_string()).as_str());

            // Gets the current date and converts it to be compatible with the ical format
            let nowstr = datetime_to_ical_str(chrono::Utc::now().naive_utc());
            // Gets the date the task was created and converts it as well
            let createdstr = datetime_to_ical_str(task.created);

            // Add metadata dates for the task
            ical_text.push_str(format!("CREATED:{}\n", createdstr).as_str());
            ical_text.push_str(format!("LAST-MODIFIED:{}\n", nowstr).as_str());
            ical_text.push_str(format!("DTSTAMP:{}\n", nowstr).as_str());

            // Adds task summary
            ical_text.push_str(format!("SUMMARY:{}\n", &task.summary).as_str());

            // Adds task due date
            if let Some(due) = task.due {
                ical_text.push_str(format!("DUE:{}\n", 
                    datetime_to_ical_str(due.and_time(chrono::NaiveTime::default()))
                ).to_string().as_str());
            }

            // Adds task priority if it's not 0
            if task.priority != 0 {
                ical_text.push_str(format!("PRIORITY:{}\n", task.priority).as_str())
            }
            // Adds task progress if it's not 0
            if task.progress != 0 {
                ical_text.push_str(format!("PERCENT-COMPLETE:{}\n", task.progress).as_str())
            }
            // Adds task's status. The completion checkbox takes precedence,
            // but if it's not checked then the task's status field is used.
            if task.completed {
                ical_text.push_str("STATUS:COMPLETED\n")
            } else {
                let mut statstr = "STATUS:".to_string();
                match task.status {
                    TaskStatus::InProgress => statstr.push_str("IN-PROGRESS\n"),
                    TaskStatus::NeedsAction => statstr.push_str("NEEDS-ACTION\n"),
                    TaskStatus::Completed => statstr.push_str("COMPLETED\n"),
                    TaskStatus::Cancelled => statstr.push_str("CANCELLED\n"),
                }
                ical_text.push_str(&statstr);
            } 
            // Adds task description if it's not empty
            if task.description != "" {
                ical_text.push_str(format!("DESCRIPTION:{}\n", &task.description.replace("\n", "\\n")).as_str());
            }
            // Ends the task data
            ical_text.push_str("END:VTODO\n");
        }
        // Ends the file
        ical_text.push_str("END:VCALENDAR\n");
        
        // Creates a file at the specified path and writes the data to it
        // Returns an error if the file already exists
        if File::open(path).is_ok() {
            return Err(std::io::ErrorKind::AlreadyExists.into());
        }
        // Both of these functions return an error if they fail
        let mut f = File::create(path)?;
        f.write_all(ical_text.as_bytes())?;
        
        // We're all good, return a reference to the file
        Ok(f)
    }
}

// Takes a chrono DateTime and converts it to a string that the calendar file can read
fn datetime_to_ical_str(datetime: chrono::NaiveDateTime) -> String {
    let datestr = datetime.date().to_string().replace("-", "");

    let mut timestr = datetime.time().to_string();
    if let Some(pos) = timestr.find(".") {
        timestr.truncate(pos);
    }
    let timestr = timestr.replace(":", "");

    format!("{}T{}", datestr, timestr)
}

// Possible errors for parsing from a file
#[derive(Debug)]
pub enum ParseFromFileError {
    InvalidFile,
    InvalidStatus,
    NonTaskItem,
    InvalidField,
}
