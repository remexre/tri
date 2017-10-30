use chrono::NaiveDate;
use diesel::update;
use diesel::prelude::*;

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use priority::Priority;
use schema::tasks;

impl Tri {
    /// Changes a task's "doneness".
    pub fn change_done(&self, id: i32, done: bool) -> Result<()> {
        let db = self.db.lock().unwrap();
        update(tasks::table.filter(tasks::id.eq(id)))
            .set(tasks::done.eq(done))
            .execute(&*db)
            .chain_err(|| ErrorKind::CouldntSetTaskDoneness(id, done))?;
        Ok(())
    }

    /// Changes a task's due date.
    pub fn change_due_date(&self, id: i32, due_date: Option<NaiveDate>) -> Result<()> {
        let db = self.db.lock().unwrap();
        update(tasks::table.filter(tasks::id.eq(id)))
            .set(tasks::due_date.eq(due_date))
            .execute(&*db)
            .chain_err(|| ErrorKind::CouldntSetTaskDueDate(id, due_date))?;
        Ok(())
    }

    /// Changes a task's "doneness".
    pub fn change_priority(&self, id: i32, priority: Priority) -> Result<()> {
        let db = self.db.lock().unwrap();
        update(tasks::table.filter(tasks::id.eq(id)))
            .set(tasks::priority.eq(priority))
            .execute(&*db)
            .chain_err(|| ErrorKind::CouldntSetTaskPriority(id, priority))?;
        Ok(())
    }
}
