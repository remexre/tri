use commands::Command;
use controller::Tri;
use errors::Result;

/// A command that asks for help.
#[derive(Debug, PartialEq)]
pub struct RemindAll;

impl Command for RemindAll {
    fn examples(_user: &str) -> Vec<String> {
        vec![]
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        warn!("{} is sending reminders to everybody!", user);
        tri.remind_all_users()?;
        Ok("Reminders sent!".to_string())
    }
}
