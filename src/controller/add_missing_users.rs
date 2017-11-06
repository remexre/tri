use std::collections::{HashMap, HashSet};

use slack_api::users as slack_users;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};

impl Tri {
    /// Adds all the Slack users that don't have accounts in the database.
    pub fn add_missing_users(&self) -> Result<()> {
        info!(target: "tri", "Checking for missing users...");

        let slack_users = slack_users::list(
            &self.slack,
            &self.slack_token,
            &slack_users::ListRequest::default(),
        ).chain_err(|| ErrorKind::CouldntGetUserList)?;
        let slack_users = slack_users.members.unwrap_or_else(Vec::new);
        let slack_user_ids = slack_users
            .iter()
            .map(|user| user.id.clone().unwrap())
            .collect::<HashSet<_>>();
        // TODO: Use display_name instead of name.
        let slack_user_names = slack_users
            .into_iter()
            .filter_map(|user| {
                let id = user.id.clone().unwrap();
                match Tri::name_for_user(user) {
                    Some(name) => Some((id, name)),
                    None => None,
                }
            })
            .collect::<HashMap<_, _>>();

        let db_user_ids = self.get_all_users()?
            .into_iter()
            .map(|user| user.slack_id)
            .collect::<HashSet<_>>();

        for user in slack_user_ids.difference(&db_user_ids).cloned() {
            let name = slack_user_names.get(&user).map(|name| name.clone());
            if let Some(name) = name.as_ref() {
                info!(target: "tri", "Adding missing user {} ({})", name, user);
            } else {
                info!(target: "tri", "Adding missing user {}", user);
            }
            self.add_user(user, name)?;
        }
        Ok(())
    }
}
