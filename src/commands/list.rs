use commands::Command;
use controller::Tri;
use errors::Result;
use render::{render_all_tasks, render_everybodys_tasks, render_my_tasks};

/// A command that lists tasks.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum List {
    /// Lists every task, including completed tasks.
    All,

    /// Lists all user's tasks.
    Everybody,

    /// Lists the user's tasks.
    Me,
}

impl Command for List {
    fn examples(_user: &str) -> Vec<String> {
        vec![
            "list all tasks".to_string(),
            "list everybody's tasks".to_string(),
            "list my tasks".to_string(),
        ]
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        let table = match *self {
            List::All => {
                let tasks = tri.get_all_tasks()?;
                render_all_tasks(tasks, tri)
            }
            List::Everybody => {
                let tasks = tri.get_all_incomplete_tasks()?;
                render_everybodys_tasks(tasks, tri)
            }
            List::Me => {
                let user = tri.must_find_user(user)?;
                let tasks = tri.get_incomplete_tasks_for(&user)?;
                render_my_tasks(tasks)
            }
        };
        Ok(format!("```\n{}```", table))
    }
}
