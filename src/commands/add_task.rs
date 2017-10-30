use chrono::NaiveDate;

use commands::Command;
use controller::Tri;
use errors::Result;
use priority::Priority;

/// A command that adds a task to a user.
#[derive(Debug, PartialEq)]
pub struct AddTask {
    /// The date (if any) the task is due by.
    pub due_date: Option<NaiveDate>,

    /// The name of the task.
    pub name: String,

    /// The priority the task has.
    pub priority: Priority,

    /// The Slack ID of the user the task is assigned to.
    pub user_slack_id: String,
}

impl Command for AddTask {
    fn examples(user: &str) -> Vec<String> {
        vec![
            format!(
                "add extreme priority task due 2001-02-03 to <@{}> buy more snacks",
                user
            ),
            format!(
                "add minimal priority task to <@{}> fix Minnehack registration",
                user
            ),
        ]
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        let assignee = tri.must_find_user(&self.user_slack_id)?;
        let task = tri.add_task(
            &assignee,
            &self.name,
            self.priority,
            self.due_date,
        )?;

        if assignee.slack_id != user {
            let msg = format!(
                "You were just assigned a task (#{}) by <@{}>.",
                task.id,
                user
            );
            tri.message_user(&assignee, &msg)?;
        }

        Ok(format!("Added as #{}.", task.id))
    }
}
