pub mod commands;
pub mod events;
pub mod message;
pub mod payload;
pub mod rich_presence;
mod shared;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Dispatch,
    Authorize,
    Subscribe,
    Unsubscribe,
    SetActivity,
    SendActivityJoinInvite,
    CloseActivityRequest,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    Ready,
    Error,
    ActivityJoin,
    ActivitySpectate,
    ActivityJoinRequest,
}

pub use self::commands::*;
pub use self::events::*;
pub use self::message::{Message, OpCode};

pub use self::rich_presence::*;

pub mod prelude {
    pub use super::commands::{Subscription, SubscriptionArgs};
    pub use super::events::{ErrorEvent, ReadyEvent};
    pub use super::rich_presence::{
        ActivityJoinEvent, ActivityJoinRequestEvent, ActivitySpectateEvent,
        CloseActivityRequestArgs, SendActivityJoinInviteArgs, SetActivityArgs,
    };
    pub use super::Command;
    pub use super::Event;
}
