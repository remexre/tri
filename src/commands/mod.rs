//! Commands that the bot can be sent.

mod add_task;
mod help;
mod list;

pub use commands::add_task::AddTask;
pub use commands::help::Help;
pub use commands::list::List;
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

    /*
    /// Changes the due date of a task.
    ChangeDueDate,

    /// Changes the priority of a task.
    ChangePriority,
    */
    /// Asks for help.
    Help(Help),

    /*
    /// Marks a task as done.
    MarkDone,
    */
    /// Lists tasks.
    List(List),

    /*
    /// Marks a task as incomplete.
    UnmarkDone,
    */
}

impl Command for DynamicCommand {
    fn examples(user: &str) -> Vec<String> {
        let mut v = vec![];
        v.extend(AddTask::examples(user));
        v.extend(Help::examples(user));
        v.extend(List::examples(user));
        v.sort();
        v
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        match *self {
            DynamicCommand::AddTask(ref cmd) => cmd.run(tri, user),
            DynamicCommand::Help(ref cmd) => cmd.run(tri, user),
            DynamicCommand::List(ref cmd) => cmd.run(tri, user),
        }
    }
}
