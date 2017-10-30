use diesel::insert;
use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::{NewUser, User};
use schema::{last_insert_rowid, users};

impl Tri {
    /// Adds a new user.
    pub fn add_user(&self, slack_id: String, name: Option<String>) -> Result<User> {
        let new_user = NewUser {
            slack_id: &slack_id,
            name: name.as_ref().map(String::as_str),
        };
        let db = self.db.lock().unwrap();
        insert(&new_user)
            .into(users::table)
            .execute(&*db)
            .chain_err(|| {
                warn!("1");
                ErrorKind::FailedAddingUser(slack_id.clone(), name.clone())
            })?;
        let id: i32 = users::table
            .select(last_insert_rowid)
            .first(&*db)
            .chain_err(|| {
                warn!("2");
                ErrorKind::FailedAddingUser(slack_id.clone(), name.clone())
            })?;
        users::table
            .filter(users::id.eq(id))
            .first(&*db)
            .chain_err(|| {
                warn!("3");
                ErrorKind::FailedAddingUser(slack_id.clone(), name.clone())
            })
    }
}
