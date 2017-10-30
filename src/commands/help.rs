use commands::{Command, DynamicCommand};
use controller::Tri;
use errors::Result;

/// A command that asks for help.
#[derive(Debug, PartialEq)]
pub struct Help;

impl Command for Help {
    fn examples(_user: &str) -> Vec<String> {
        vec!["help".to_string()]
    }

    fn run(&self, tri: &Tri, user: &str) -> Result<String> {
        let mut msg = "Try one of the following:\n".to_string();
        for cmd in DynamicCommand::examples(user) {
            msg += "\n - <@";
            msg += &tri.slack_id;
            msg += "> ";
            msg += &cmd;
        }
        Ok(msg)
    }
}
