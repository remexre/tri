use chrono::NaiveDate;

use commands::Command;
use controller::Tri;
use errors::Result;

/// A command that changes the due date for a task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChangeDueDate(pub i32, pub Option<NaiveDate>);

impl Command for ChangeDueDate {
    fn examples(_user: &str) -> Vec<String> {
        vec![
            "set due date for 1 to 2001-02-03".to_string(),
            "remove due date from 1".to_string(),
        ]
    }

    fn run(&self, tri: &Tri, _user: &str) -> Result<String> {
        tri.change_due_date(self.0, self.1)?;
        Ok("Due date changed.".to_string())
    }
}
