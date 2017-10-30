mod add_missing_users;
mod add_task;
mod add_user;
mod find_user;
mod get_all_users;
mod get_tasks;
mod message_user;
mod name_for_id;
mod remind_all_users;
mod remind_user;
mod rtm;
mod update_task;

use std::env;
use std::sync::Mutex;

use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use slack::RtmClient;
use slack::api::auth::test;
use slack::api::requests::Client;
use void::Void;

use errors::{ErrorKind, Result, ResultExt};

/// The main controller object for the library.
pub struct Tri {
    db: Mutex<SqliteConnection>,
    slack: Client,
    slack_token: String,

    /// The Slack ID of the bot user.
    pub slack_id: String,
}

impl Tri {
    /// Initializes the controller.
    pub fn new(db_url: &str, slack_token: String) -> Result<Tri> {
        let db = Mutex::new(SqliteConnection::establish(db_url).chain_err(|| {
            ErrorKind::CouldntConnectToDb
        })?);

        let slack = Client::new().chain_err(|| ErrorKind::CouldntConnectToSlack)?;

        let slack_id = test(&slack, &slack_token)
            .chain_err(|| ErrorKind::CouldntConnectToSlack)?
            .user_id
            .expect("We didn't get a user_id from Slack?");

        Ok(Tri {
            db,
            slack,
            slack_id,
            slack_token,
        })
    }

    /// Initializes the controller from environment variables.
    pub fn new_env() -> Result<Tri> {
        let db_url = env::var("DATABASE_URL").chain_err(|| {
            ErrorKind::MissingEnvVar("DATABASE_URL")
        })?;
        let slack_token = env::var("SLACK_API_TOKEN").chain_err(|| {
            ErrorKind::MissingEnvVar("SLACK_API_TOKEN")
        })?;
        Tri::new(&db_url, slack_token)
    }

    /// Starts listening for messages. This will block.
    pub fn listen(&self) -> Result<Void> {
        let rtm = RtmClient::login(&self.slack_token).chain_err(|| {
            ErrorKind::CouldntConnectToSlack
        })?;

        let mut handler = rtm::Handler(self);
        loop {
            if let Err(err) = rtm.run(&mut handler) {
                error!("{}", err);
            }
        }
    }
}
