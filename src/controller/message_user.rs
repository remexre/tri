use slack::api::chat::{post_message, PostMessageRequest};
use slack::api::im::{open, OpenRequest};

use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};
use models::User;

impl Tri {
    /// Sends a direct message to the user.
    pub fn message_user(&self, user: &User, msg: &str) -> Result<()> {
        let req = OpenRequest {
            user: &user.slack_id,
            return_im: Some(false),
        };
        let channel = open(&self.slack, &self.slack_token, &req)
            .chain_err(|| ErrorKind::CouldntSendUserMessage(user.clone()))?
            .channel
            .and_then(|im| im.id)
            .ok_or_else(|| ErrorKind::CouldntSendUserMessage(user.clone()))?;

        let req = PostMessageRequest {
            channel: &channel,
            text: msg,
            as_user: Some(true),
            ..PostMessageRequest::default()
        };
        post_message(&self.slack, &self.slack_token, &req)
            .chain_err(|| ErrorKind::CouldntSendUserMessage(user.clone()))?;
        Ok(())
    }
}
