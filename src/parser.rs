// ----------------------------------------------------------------------------
// Functionality for parsing a TaskList to or from an iCal file with VTODOs.
// ----------------------------------------------------------------------------

use std::{fs::File, io, path::Path};

use eframe::egui;

use crate::task::*;

impl TaskList {
    /// Converts an iCal file to a TaskList.
    ///
    /// Example:
    /// ```
    /// use taskmaster_rs::task::*;
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// use eframe::egui::Color32;
    ///
    /// let list = TaskList::from_ical_file(Path::new("test.ics")).unwrap();
    ///
    /// assert_eq!(
    ///     list,
    ///     TaskList {
    ///         name: "test".to_string(),
    ///         tasks: vec![Task {
    ///             summary: "Task 1".to_string(),
    ///             completed: false,
    ///             description: "description\n".to_string(),
    ///             progress: 47,
    ///             priority: 9,
    ///             status: TaskStatus::NeedsAction,
    ///             due: Some(chrono::NaiveDate::parse_from_str("20230825", "%Y%m%d").unwrap()),
    ///             show_modal: false,
    ///             created: chrono::NaiveDateTime::parse_from_str("20230801151208", "%Y%m%d%H%M%S").unwrap()
    ///         }],
    ///         color: Color32::from_rgb(83, 130, 163)
    ///     }
    /// );
    /// ```
    pub fn from_ical_file(path: &Path) -> Result<TaskList, ParseFromFileError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Err(ParseFromFileError::InvalidFile),
        };

        let reader = io::BufReader::new(file);

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
                        // Conver ical hex color to rgb color that can be stored in a Task
                        if let Ok(rgb) = colorsys::Rgb::from_hex_str(&value) {
                            list.color = egui::Color32::from_rgb(
                                rgb.red().round() as u8,
                                rgb.green().round() as u8,
                                rgb.blue().round() as u8,
                            );
                        }
                    }
                    // Checks for a BEGIN statement
                    "BEGIN" => {
                        match value.as_str() {
                            // If it's starting a new task, add a task to the vector of tasks
                            "VTODO" => tasks.push(Task::default()),
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
                        if let Ok(date) =
                            chrono::NaiveDateTime::parse_from_str(&datestr, "%Y%m%dT%H%M%S")
                        {
                            tasks[task_counter].due = Some(date.date());
                        } else {
                            return Err(ParseFromFileError::InvalidField);
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
                            }
                            "CANCELLED" => TaskStatus::Cancelled,
                            _ => return Err(ParseFromFileError::InvalidStatus),
                        }
                    }
                    // Set the currently addressed task's description
                    "DESCRIPTION" => {
                        tasks[task_counter].description = value.replace("\\n", "\n");
                    }
                    // If the file says that the task description is complete,
                    // iterate the task counter so that it can be used to address
                    // the task that is added next (if any).
                    "END" => {
                        if value == *"VTODO".to_string() {
                            task_counter += 1;
                        }
                    }
                    // Store the task's creation date
                    "CREATED" => {
                        let datestr = value;
                        if let Ok(date) =
                            chrono::NaiveDateTime::parse_from_str(&datestr, "%Y%m%dT%H%M%S")
                        {
                            tasks[task_counter].created = date;
                        } else {
                            return Err(ParseFromFileError::InvalidField);
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

    /// Converts a TaskList to a string containing the contents of a potential iCal file.
    /// This lets whatever is implementing the function handle writing it to a file,
    /// or using the string for any other purpose.
    ///
    /// Example:
    /// ```
    /// use taskmaster_rs::task::*;
    /// use std::path::Path;
    /// use std::fs;
    ///
    /// use eframe::egui::Color32;
    ///
    /// let liststr = TaskList::from_ical_file(Path::new("test.ics")).unwrap()
    ///     .to_ical_string();
    ///
    /// println!("{}", liststr.trim());
    /// assert!(
    /// // WildMatch lets you check if two strings are matching non-exactly using wildcards,
    /// // which is useful here because the UUID and creation time of a task are different each time
    ///     wildmatch::WildMatch::new(
    ///"BEGIN:VCALENDAR
    ///VERSION:2.0
    ///CALSCALE:GREGORIAN
    ///PRODID:-//taskmaster-rs//github.com//
    ///X-WR-CALNAME:test
    ///X-APPLE-CALENDAR-COLOR:#5382A3
    ///REFRESH-INTERVAL;VALUE=DURATION:PT4H
    ///X-PUBLISHED-TTL:PT4H
    ///BEGIN:VTODO
    ///UID:????????-????-????-????-????????????
    ///CREATED:20230801T151208
    ///LAST-MODIFIED:????????T??????
    ///DTSTAMP:????????T??????
    ///SUMMARY:Task 1
    ///DUE:20230825T000000
    ///PRIORITY:9
    ///PERCENT-COMPLETE:47
    ///STATUS:NEEDS-ACTION
    ///DESCRIPTION:description\\n
    ///END:VTODO
    ///END:VCALENDAR").matches(liststr.trim())
    /// );
    /// ```
    pub fn to_ical_string(&self) -> String {
        // Initiate text that will eventually be added to the calendar file
        // As well as adding some initial variables via a format string
        let mut ical_text = format!(
            "BEGIN:VCALENDAR
VERSION:2.0
CALSCALE:GREGORIAN
PRODID:-//taskmaster-rs//github.com//
X-WR-CALNAME:{}
X-APPLE-CALENDAR-COLOR:{}
REFRESH-INTERVAL;VALUE=DURATION:PT4H
X-PUBLISHED-TTL:PT4H",
            // Now the variables that are substituted into the {}s are specified
            self.name,
            // Convert the TaskList's color to hexadecimal and insert it into the string
            // {:X} in a format string changes decimal numbers to hexadecimal
            format_args!(
                "#{:X}{:X}{:X}",
                self.color.r(),
                self.color.g(),
                self.color.b()
            )
        );

        // Add data for every todo item
        for task in &self.tasks {
            // Begins the task data
            ical_text.push_str("\nBEGIN:VTODO\n");
            // Generates a unique UID for the task
            ical_text.push_str(format!("UID:{}\n", task.uuid).as_str());

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
                ical_text.push_str(
                    format!(
                        "DUE:{}\n",
                        datetime_to_ical_str(due.and_time(chrono::NaiveTime::default()))
                    )
                    .to_string()
                    .as_str(),
                );
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
            if !task.description.is_empty() {
                ical_text.push_str(
                    format!("DESCRIPTION:{}\n", &task.description.replace('\n', "\\n")).as_str(),
                );
            }
            // Ends the task data
            ical_text.push_str("END:VTODO\n");
        }
        // Ends the file
        ical_text.push_str("END:VCALENDAR\n");

        // We're all good, return a reference to the file
        ical_text
    }
}

/// Takes a chrono DateTime and converts it to a string that the calendar file can read.
///
/// Examples:
/// ```
/// use taskmaster_rs::parser::*;
///
/// let date1 = chrono::NaiveDateTime::parse_from_str("2023-08-08 13:08:20", "%Y-%m-%d %H:%M:%S");
/// let date2 = chrono::NaiveDateTime::parse_from_str("1337-06-09 04:20:00", "%Y-%m-%d %H:%M:%S");
/// let date3 = chrono::NaiveDateTime::from_timestamp_millis(0); // Unix epoch, or 1st Jan 1970 00:00
/// assert_eq!(datetime_to_ical_str(date1.unwrap()), "20230808T130820");
/// assert_eq!(datetime_to_ical_str(date2.unwrap()), "13370609T042000");
/// assert_eq!(datetime_to_ical_str(date3.unwrap()), "19700101T000000");
/// ```
pub fn datetime_to_ical_str(datetime: chrono::NaiveDateTime) -> String {
    datetime.format("%Y%m%dT%H%M%S").to_string()
}

// Possible errors for parsing from a file
#[derive(Debug)]
pub enum ParseFromFileError {
    InvalidFile,
    InvalidStatus,
    NonTaskItem,
    InvalidField,
}
