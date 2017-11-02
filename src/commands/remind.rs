use commands::Command;
use controller::Tri;
use errors::Result;

/// A command that asks for help.
#[derive(Debug, PartialEq)]
pub struct Remind(pub Option<String>);

impl Command for Remind {
    fn examples(user: &str) -> Vec<String> {
        vec![format!("remind <@{}>", user)]
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        if let Some(ref to_user) = self.0 {
            let to_user = tri.must_find_user(to_user)?;
            tri.remind_user(&to_user)?;
            Ok("Reminder sent!".to_string())
        } else {
            warn!("{} is sending reminders to everybody!", user);
            tri.remind_all_users()?;
            Ok("Reminders sent!".to_string())
        }
    }
}
