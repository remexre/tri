//! The database models.

use std::fmt::{Display, Formatter, Result as FmtResult};

use chrono::NaiveDate;

use priority::Priority;
use schema::{tasks, users};

/// A task.
#[derive(Clone, Debug, Queryable)]
pub struct Task {
    /// The task's database ID.
    pub id: i32,

    /// The name of the task.
    pub name: String,

    /// The creation date of the task.
    pub create_date: NaiveDate,

    /// The due date of the task.
    pub due_date: Option<NaiveDate>,

    /// The priority of the task.
    pub priority: Priority,

    /// The ID of the user the task is assigned to.
    pub user_id: i32,

    /// Whether the task is completed or not.
    pub done: bool,
}

/// A user.
#[derive(Clone, Debug, Queryable)]
pub struct User {
    /// The user's database ID.
    pub id: i32,

    /// The user's ID on Slack, for example `U7RD06U1G`.
    pub slack_id: String,

    /// The user's name. This is essentially a comment field, and may be
    /// ignored.
    pub name: Option<String>,
}

impl Display for User {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        if let Some(name) = self.name.as_ref() {
            write!(fmt, "{} ({})", name, self.slack_id)
        } else {
            write!(fmt, "{}", self.slack_id)
        }
    }
}

/// A task to be added to the database.
#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub(crate) struct NewTask<'a> {
    /// The name of the task.
    pub name: &'a str,

    /// The due date of the task.
    pub due_date: Option<NaiveDate>,

    /// The priority of the task.
    pub priority: Priority,

    /// The ID of the user the task is assigned to.
    pub user_id: i32,
}

/// A user to be added to the database.
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub(crate) struct NewUser<'a> {
    /// The user's ID on Slack, for example `U7RD06U1G`.
    pub slack_id: &'a str,

    /// The user's name. This is essentially a comment field, and may be
    /// ignored.
    pub name: Option<&'a str>,
}
