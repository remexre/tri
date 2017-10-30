use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::{Task, User};
use schema::tasks;

impl Tri {
    /// Returns every task in history, sorted by database ID.
    pub fn get_all_tasks(&self) -> Result<Vec<Task>> {
        let db = self.db.lock().unwrap();
        tasks::table.order(tasks::id.asc()).load(&*db).chain_err(
            || {
                ErrorKind::CouldntGetIncompleteTasks
            },
        )
    }

    /// Returns the tasks that still need to be completed, sorted by priority
    /// (highest to lowest).
    pub fn get_all_incomplete_tasks(&self) -> Result<Vec<Task>> {
        let db = self.db.lock().unwrap();
        tasks::table
            .filter(tasks::done.eq(false))
            .order(tasks::priority.desc())
            .load(&*db)
            .chain_err(|| ErrorKind::CouldntGetIncompleteTasks)
    }

    /// Returns the tasks the user still needs to complete, sorted by priority
    /// (highest to lowest).
    pub fn get_incomplete_tasks_for(&self, user: &User) -> Result<Vec<Task>> {
        let db = self.db.lock().unwrap();
        tasks::table
            .filter(tasks::user_id.eq(user.id))
            .filter(tasks::done.eq(false))
            .order(tasks::priority.desc())
            .load(&*db)
            .chain_err(|| ErrorKind::CouldntGetIncompleteTasksForUser(user.clone()))
    }
}
