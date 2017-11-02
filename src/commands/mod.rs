//! Commands that the bot can be sent.

mod add_task;
mod change_done;
mod change_due_date;
mod change_priority;
mod help;
mod list;
mod remind;

pub use commands::add_task::AddTask;
pub use commands::change_done::ChangeDone;
pub use commands::change_due_date::ChangeDueDate;
pub use commands::change_priority::ChangePriority;
pub use commands::help::Help;
pub use commands::list::List;
pub use commands::remind::Remind;
use controller::Tri;
use errors::Result;

/// The command trait.
pub trait Command {
    /// Returns an example command.
    ///
    /// The parameter is the Slack ID of a valid user.
    fn examples(user: &str) -> Vec<String>;

    /// Runs the command, returning the output to respond with.
    fn run(&self, tri: &Tri, user: &str) -> Result<String>;
}

/// An enum for any supported command.
#[derive(Debug, PartialEq)]
pub enum DynamicCommand {
    /// Adds a task to a user.
    AddTask(AddTask),

    /// Changes the "doneness" of a task.
    ChangeDone(ChangeDone),

    /// Changes the due date of a task.
    ChangeDueDate(ChangeDueDate),

    /// Changes the priority of a task.
    ChangePriority(ChangePriority),

    /// Asks for help.
    Help(Help),

    /// Lists tasks.
    List(List),

    /// Sends reminders to people.
    Remind(Remind),
}

impl Command for DynamicCommand {
    fn examples(user: &str) -> Vec<String> {
        let mut v = vec![];
        v.extend(AddTask::examples(user));
        v.extend(ChangeDone::examples(user));
        v.extend(ChangeDueDate::examples(user));
        v.extend(ChangePriority::examples(user));
        v.extend(Help::examples(user));
        v.extend(List::examples(user));
        v.extend(Remind::examples(user));
        v.sort();
        v
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        match *self {
            DynamicCommand::AddTask(ref cmd) => cmd.run(tri, user),
            DynamicCommand::ChangeDone(ref cmd) => cmd.run(tri, user),
            DynamicCommand::ChangeDueDate(ref cmd) => cmd.run(tri, user),
            DynamicCommand::ChangePriority(ref cmd) => cmd.run(tri, user),
            DynamicCommand::Help(ref cmd) => cmd.run(tri, user),
            DynamicCommand::List(ref cmd) => cmd.run(tri, user),
            DynamicCommand::Remind(ref cmd) => cmd.run(tri, user),
        }
    }
}
