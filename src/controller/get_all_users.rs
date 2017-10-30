use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::User;
use schema::users;

impl Tri {
    /// Returns a list of all the users in the database.
    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let db = self.db.lock().unwrap();
        users::table
            .select(users::all_columns)
            .load(&*db)
            .chain_err(|| ErrorKind::CouldntGetUsers)
    }
}
