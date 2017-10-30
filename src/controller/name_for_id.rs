use diesel::update;
use diesel::prelude::*;
use slack::api::users::{info, InfoRequest};

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use schema::users;

impl Tri {
    /// Returns the Slack username for the given database user ID.
    ///
    /// If `network` is `true` and no name is found in the database, the
    /// username will be looked up from Slack.
    pub fn name_for_id(&self, id: i32, network: bool) -> Result<String> {
        let db = self.db.lock().unwrap();

        let (slack_id, name) = users::table
            .filter(users::id.eq(id))
            .select((users::slack_id, users::name))
            .first(&*db)
            .chain_err(|| ErrorKind::FailedFindingUserByDbId(id))?;
        if let Some(name) = name {
            Ok(name)
        } else if network {
            let name = self.network_lookup_name(slack_id)?;
            let r = update(users::table.filter(users::id.eq(id)))
                .set(users::name.eq(&name))
                .execute(&*db);
            if let Err(err) = r {
                error!("{}", err);
            }
            Ok(name)
        } else {
            Err(ErrorKind::FailedFindingUserByDbId(id).into())
        }
    }

    /// Looks up the name for a Slack ID.
    ///
    /// TODO: Use display_name instead of name.
    fn network_lookup_name(&self, slack_id: String) -> Result<String> {
        let req = InfoRequest { user: &slack_id };
        info(&self.slack, &self.slack_token, &req)
            .chain_err(|| ErrorKind::FailedGettingUserName(slack_id.clone()))?
            .user
            .and_then(|user| user.name)
            .ok_or_else(|| ErrorKind::FailedGettingUserName(slack_id.clone()).into())
    }
}
