use diesel::update;
use diesel::prelude::*;
use slack_api::User as SlackUser;
use slack_api::users::{info, InfoRequest};

use controller::Tri;
use errors::{Error, ErrorKind, Result, ResultExt};
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
            .and_then(Tri::name_for_user)
            .ok_or_else(|| Error::from("User has no name"))
            .chain_err(|| ErrorKind::FailedGettingUserName(slack_id.clone()))
    }

    /// Gets the name from a Slack User object.
    pub(crate) fn name_for_user(user: SlackUser) -> Option<String> {
        fn n(a: Option<String>) -> Option<String> {
            match a {
                Some(s) => if s == "" { None } else { Some(s) },
                None => None,
            }
        }
        user.profile
            .and_then(|p| n(p.display_name_normalized).or(n(p.display_name)))
            .or(n(user.name))
            .or(n(user.real_name))
    }
}
