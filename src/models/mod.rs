mod shared;
pub mod message;
pub mod payload;
pub mod commands;
pub mod events;
pub mod rich_presence;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Dispatch,
    Authorize,
    Subscribe,
    Unsubscribe,
    #[cfg(feature = "rich_presence")]
    SetActivity,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    Ready,
    Error,
    #[cfg(feature = "rich_presence")]
    ActivityJoin,
    #[cfg(feature = "rich_presence")]
    ActivitySpectate,
    #[cfg(feature = "rich_presence")]
    ActivityJoinRequest,
}

pub use self::message::{Message, OpCode};
pub use self::commands::*;
pub use self::events::*;

#[cfg(feature = "rich_presence")]
pub use self::rich_presence::*;

pub mod prelude {
    pub use super::Command;
    pub use super::Event;
    #[cfg(feature = "rich_presence")]
    pub use super::rich_presence::{
        SetActivityArgs,
        ActivityJoinEvent,
        ActivitySpectateEvent,
        ActivityJoinRequestEvent
    };
    pub use super::commands::{
        SubscriptionArgs, Subscription
    };
    pub use super::events::{
        ReadyEvent,
        ErrorEvent,
    };
}
