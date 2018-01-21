use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::User;
use schema::users;

impl Tri {
    /// Finds the user with the given Slack ID, returning `None` if they do not
    /// exist.
    pub fn find_user(&self, slack_id: &str) -> Result<Option<User>> {
        let db = self.db.lock().unwrap();
        users::table
            .filter(users::slack_id.eq(slack_id))
            .first(&*db)
            .optional()
            .chain_err(|| ErrorKind::FailedFindingUser(slack_id.to_string()))
    }

    /// Finds the user with the given Slack ID, giving an error if they do not
    /// exist.
    pub fn must_find_user(&self, slack_id: &str) -> Result<User> {
        self.find_user(slack_id)?
            .ok_or_else(|| ErrorKind::NoSuchUser(slack_id.to_string()).into())
    }
}
