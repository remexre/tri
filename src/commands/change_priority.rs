use commands::Command;
use controller::Tri;
use errors::Result;
use priority::Priority;

/// A command that changes the priority for a task.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChangePriority(pub i32, pub Priority);

impl Command for ChangePriority {
    fn examples(_user: &str) -> Vec<String> {
        vec![
            "set priority for 1 to high".to_string(),
            "set priority for 3 to medium".to_string(),
            "set priority for 2 to low".to_string(),
        ]
    }

    fn run(&self, tri: &Tri, _user: &str) -> Result<String> {
        tri.change_priority(self.0, self.1)?;
        Ok("Priority changed.".to_string())
    }
}
