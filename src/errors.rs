#![allow(missing_docs)]

use chrono::NaiveDate;

use models::User;
use priority::Priority;

error_chain! {
    errors {
        CouldntConnectToDb {
            description("Couldn't connect to the database")
            display("Couldn't connect to the database")
        }
        CouldntConnectToSlack {
            description("Couldn't connect to Slack")
            display("Couldn't connect to Slack")
        }
        CouldntGetTasks {
            description("Couldn't get a list of tasks")
            display("Couldn't get a list of tasks")
        }
        CouldntGetIncompleteTasks {
            description("Couldn't get a list of incomplete tasks")
            display("Couldn't get a list of incomplete tasks")
        }
        CouldntGetIncompleteTasksForUser(user: User) {
            description("Couldn't get a list of incomplete tasks for a user")
            display("Couldn't get a list of incomplete tasks for {}",
                user.name.as_ref().unwrap_or(&user.slack_id))
        }
        CouldntGetUserList {
            description("Couldn't get a list of Slack users")
            display("Couldn't get a list of Slack users")
        }
        CouldntGetUsers {
            description("Couldn't get a list of users")
            display("Couldn't get a list of users")
        }
        CouldntSendChannelMessage(channel: String) {
            description("Couldn't send a message")
            display("Couldn't send a message to the channel {}", channel)
        }
        CouldntSendUserMessage(user: User) {
            description("Couldn't send a message")
            display("Couldn't send a message to the user {}", user)
        }
        FailedAddingTask(user: User, name: String, priority: Priority, due_date: Option<NaiveDate>) {
            description("A task could not be added")
            display("A task with with priority {}, due {}, assigned to {}, with the name {:?} could not be added",
                priority,
                match *due_date {
                    Some(ref date) => date.to_string(),
                    None => "at an indeterminate date".to_string(),
                },
                user,
                name)
        }
        FailedAddingUser(slack_id: String, name: Option<String>) {
            description("A user could not be added")
            display("The user with Slack ID {} {}could not be added",
                slack_id,
                match *name {
                    Some(ref name) => format!("and name `{}' ", name),
                    None => "".to_string(),
                })
        }
        FailedFindingUser(slack_id: String) {
            description("A user could not be found")
            display("The user with Slack ID {} could not be found", slack_id)
        }
        FailedFindingUserByDbId(id: i32) {
            description("A user could not be found")
            display("The user with DB ID {} could not be found", id)
        }
        FailedGettingUserName(slack_id: String) {
            description("A user's name could not be determined")
            display("The user with Slack ID {} could not have its name determined", slack_id)
        }
        FailedToListen {
            description("Listening for messages failed")
            display("Listening for messages failed")
        }
        MissingEnvVar(var: &'static str) {
            description("A required environment variable was missing")
            display("The environment variable {} was missing", var)
        }
        NoSuchUser(slack_id: String) {
            description("A user couldn't be found")
            display("The user with Slack ID {} couldn't be found", slack_id)
        }
    }
}
