use chrono::{DateTime, UTC};
use slack::{Error, Event, EventHandler, RtmClient};

#[derive(Debug)]
pub struct Bot {
    pub last_ping: Option<DateTime<UTC>>,
}

//let _ = client.send_message("#test", "Hello world! (rtm)");
//let _ = client.post_message("#test", "hello world! (postMessage)", None);
//let _ = client.set_topic("#test", "bots rule!");

impl EventHandler for Bot {
    fn on_event(&mut self, _client: &mut RtmClient, event: Result<Event, Error>, _json: &str) {
        println!("{:?}", event);
        use::slack::Event::*;
        match event.unwrap() {
            Hello => {},
            Message(_msg) => {},
            UserTyping { .. } => {},
            ChannelMarked { .. } => {},
            ChannelCreated { .. } => {},
            ChannelJoined { .. } => {},
            ChannelLeft { .. } => {},
            ChannelDeleted { .. } => {},
            ChannelRename { .. } => {},
            ChannelArchive { .. } => {},
            ChannelUnArchive { ..} => {},
            ChannelHistoryChanged { .. } => {},
            ImCreated { .. } => {},
            ImOpen { .. } => {},
            ImClose { .. } => {},
            ImMarked { .. } => {},
            ImHistoryChanged { .. } => {},
            GroupJoined { .. } => {},
            GroupLeft { .. } => {},
            GroupOpen { .. } => {},
            GroupClose { .. } => {},
            GroupArchive { .. } => {},
            GroupUnArchive { .. } => {},
            GroupRename { .. } => {},
            GroupMarked { .. } => {},
            GroupHistoryChanged { .. } => {},
            FileCreated { .. } => {},
            FileShared { .. } => {},
            FileUnShared { .. } => {},
            FilePublic { .. } => {},
            FilePrivate { .. } => {},
            FileChange { .. } => {},
            FileDeleted { .. } => {},
            FileCommentAdded { .. } => {},
            FileCommentEdited { .. } => {},
            FileCommentDeleted { .. } => {},
            PinAdded { .. } => {},
            PinRemoved { .. } => {},
            PresenceChange { .. } => {},
            ManualPresenceChange { .. } => {},
            PrefChange { .. } => {},
            UserChange { .. } => {},
            TeamJoin { .. } => {},
            StarAdded { .. } => {},
            StarRemoved { .. } => {},
            ReactionAdded { .. } => {},
            ReactionRemoved { .. } => {},
            EmojiChanged { .. } => {},
            CommandsChanged { .. } => {},
            TeamPlanChange { .. } => {},
            TeamPrefChange { .. } => {},
            TeamRename { .. } => {},
            TeamDomainChange { .. } => {},
            EmailDomainChanged { .. } => {},
            BotAdded { .. } => {},
            BotChanged { .. } => {},
            AccountsChanged => {},
            TeamMigrationStarted => {},
            ReconnectUrl => {},
            MessageSent { .. } => {},
            MessageError { .. } => {},
        }
    }

    fn on_connect(&mut self, _client: &mut RtmClient) {}

    fn on_close(&mut self, _client: &mut RtmClient) {}

    fn on_ping(&mut self, _client: &mut RtmClient) {
        self.last_ping = Some(UTC::now());
    }
}
