use controller::Tri;
use errors::Result;

impl Tri {
    /// Sends reminders to all users.
    pub fn remind_all_users(&self) -> Result<()> {
        for user in self.get_all_users()? {
            self.remind_user(&user)?;
        }
        Ok(())
    }
}
