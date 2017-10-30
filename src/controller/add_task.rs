use chrono::NaiveDate;
use diesel::insert;
use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::{NewTask, Task, User};
use priority::Priority;
use schema::{last_insert_rowid, tasks};

impl Tri {
    /// Adds a new task. Does not inform the user.
    pub fn add_task(
        &self,
        user: &User,
        name: &str,
        priority: Priority,
        due_date: Option<NaiveDate>,
    ) -> Result<Task> {
        let new_task = NewTask {
            name,
            due_date,
            priority,
            user_id: user.id,
        };
        let db = self.db.lock().unwrap();
        insert(&new_task)
            .into(tasks::table)
            .execute(&*db)
            .chain_err(|| {
                ErrorKind::FailedAddingTask(user.clone(), name.to_string(), priority, due_date)
            })?;
        let id: i32 = tasks::table
            .select(last_insert_rowid)
            .first(&*db)
            .chain_err(|| {
                ErrorKind::FailedAddingTask(user.clone(), name.to_string(), priority, due_date)
            })?;
        tasks::table
            .filter(tasks::id.eq(id as i32))
            .first(&*db)
            .chain_err(|| {
                ErrorKind::FailedAddingTask(user.clone(), name.to_string(), priority, due_date)
            })
    }
}
