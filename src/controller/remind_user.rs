use controller::Tri;
use errors::Result;
use models::User;
use render::render_my_tasks;

impl Tri {
    /// Sends reminders about incomplete tasks to a user.
    pub fn remind_user(&self, user: &User) -> Result<()> {
        let tasks = self.get_incomplete_tasks_for(user)?;
        if tasks.len() == 0 {
            return Ok(());
        }

        info!(target: "tri", "Sending a task reminder to {}", user);
        let msg = format!(
            "You have the following tasks assigned:\n```\n{}```",
            render_my_tasks(tasks)
        );
        self.message_user(user, &msg)
    }
}
