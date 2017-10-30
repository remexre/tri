use commands::Command;
use controller::Tri;
use errors::Result;

/// A command that changes the "doneness" for a task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChangeDone(pub i32, pub bool);

impl Command for ChangeDone {
    fn examples(_user: &str) -> Vec<String> {
        vec![
            "done 1".to_string(),
            "set done 2".to_string(),
            "unset done 3".to_string(),
        ]
    }

    fn run(&self, tri: &Tri, _user: &str) -> Result<String> {
        tri.change_done(self.0, self.1)?;
        Ok(format!(
            "Task marked as {}.",
            if self.1 { "complete" } else { "incomplete" }
        ))
    }
}
