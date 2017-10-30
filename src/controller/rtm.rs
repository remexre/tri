use error_chain::ChainedError;
use slack::{Event, EventHandler, Message, RtmClient, Sender};

use commands::{Command, DynamicCommand, Help};
use controller::Tri;
use errors::{ErrorKind, Result, ResultExt};

pub struct Handler<'a>(pub &'a Tri);

impl<'a> Handler<'a> {
    fn handle(&self, sender: &Sender, text: String, user: String, channel: String) -> Result<()> {
        let at_me = format!("<@{}> ", self.0.slack_id);
        if !text.starts_with(&at_me) || user == self.0.slack_id {
            debug!("Ignoring.");
            return Ok(());
        }
        let text = &text[at_me.len()..];

        sender.send_typing(&channel).ok();

        match text.parse::<DynamicCommand>() {
            Ok(cmd) => {
                let res = match cmd.run(self.0, &user) {
                    Ok(res) => res,
                    Err(err) => {
                        error!("{}", err.display_chain());
                        format!("An error occurred:\n\n```\n{}\n```", err)
                    }
                };
                sender.send_message(&channel, &res).chain_err(|| {
                    ErrorKind::CouldntSendChannelMessage(channel)
                })?;
            }
            Err(()) => {
                let msg = format!("Huh? I didn't understand that.\n{}",
                    Help.run(self.0, &user).unwrap());
                sender.send_message(&channel, &msg)
                    .chain_err(|| ErrorKind::CouldntSendChannelMessage(channel))?;
            }
        }
        Ok(())
    }
}

impl<'a> EventHandler for Handler<'a> {
    fn on_connect(&mut self, _rtm: &RtmClient) {
        info!("Connected to Slack!");
    }

    fn on_close(&mut self, _rtm: &RtmClient) {
        warn!("Slack connection closed.");
    }

    fn on_event(&mut self, rtm: &RtmClient, event: Event) {
        debug!("Got event from Slack: {:?}", event);
        match event {
            Event::Message(msg) => {
                if let Message::Standard(msg) = *msg {
                    if let (Some(channel), Some(text), Some(user)) =
                        (msg.channel, msg.text, msg.user)
                    {
                        info!(
                            "Got message {:?} from {} in channel {}",
                            text,
                            user,
                            channel
                        );
                        if let Err(err) = self.handle(rtm.sender(), text, user, channel) {
                            error!("{}", err.display_chain());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
