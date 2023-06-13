/// The Discord commands module
pub mod commands;
/// The events module
pub mod events;
/// The module to handle messages
pub mod message;
/// The module to handle payloads
pub mod payload;
/// The rich presence module
pub mod rich_presence;
mod shared;

/// Different Discord commands
#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    /// Dispatch something to Discord
    Dispatch,
    /// Authorize connection
    Authorize,
    /// Subscribe to an event
    Subscribe,
    /// Unsubscribe from Discord
    Unsubscribe,
    /// Set the current user's activity
    SetActivity,
    /// Send an invite to join a game
    SendActivityJoinInvite,
    /// Close the invite to join a game
    CloseActivityRequest,
}

/// Discord events
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Copy, Clone, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    /// Ready event, fired when the client is ready, but not if an error occurs
    Ready,
    /// Error event, overrides the `Ready` event
    Error,
    /// ActivityJoin event, fired when the client's game is joined by a player
    ActivityJoin,
    /// ActivitySpectate event, fired when the client receives a spectate request
    ActivitySpectate,
    /// ActivityJoinRequest event, fired when the client receives a join request
    ActivityJoinRequest,
}

pub use commands::*;
pub use events::*;
pub use message::{Message, OpCode};

pub use rich_presence::*;

/// Prelude for all Discord RPC types
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
