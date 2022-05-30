use crate::models::rich_presence::{
    Activity, CloseActivityRequestArgs, SendActivityJoinInviteArgs, SetActivityArgs,
};
use crate::{
    connection::Manager as ConnectionManager,
    event_handler::{Context as EventContext, HandlerRegistry},
    models::{
        commands::{Subscription, SubscriptionArgs},
        message::Message,
        payload::Payload,
        Command, Event, OpCode,
    },
    DiscordError, Result,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

macro_rules! event_handler_function {
    ( $( $name:ident, $event:expr ),* ) => {
        event_handler_function!{@gen $([ $name, $event, concat!("Listens for the `", stringify!($event), "` event")])*}
    };

    (@gen $( [ $name:ident, $event:expr, $doc:expr ] ), *) => {
        $(
            #[doc = $doc]
            pub fn $name<F>(&mut self, handler: F)
                where F: Fn(EventContext) + 'static + Send + Sync
            {
                self.on_event($event, handler);
            }
        )*
    }
}

/// The Discord client
#[derive(Clone)]
pub struct Client {
    connection_manager: ConnectionManager,
    event_handler_registry: HandlerRegistry<'static>,
}

impl Client {
    /// Creates a new `Client`
    pub fn new(client_id: u64) -> Self {
        let event_handler_registry = HandlerRegistry::new();
        let connection_manager = ConnectionManager::new(client_id, event_handler_registry.clone());
        Self {
            connection_manager,
            event_handler_registry,
        }
    }

    /// Start the client and connect to Discord
    pub fn start(&mut self) {
        self.connection_manager.start();
    }

    fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
    where
        A: Serialize + Send + Sync,
        E: Serialize + DeserializeOwned + Send + Sync,
    {
        let message = Message::new(
            OpCode::Frame,
            Payload::with_nonce(cmd, Some(args), None, evt),
        );
        self.connection_manager.send(message?)?;
        let Message { payload, .. } = self.connection_manager.recv()?;
        let response: Payload<E> = serde_json::from_str(&payload)?;

        match response.evt {
            Some(Event::Error) => Err(DiscordError::SubscriptionFailed),
            _ => Ok(response),
        }
    }

    /// Set the users current activity
    pub fn set_activity<F>(&mut self, f: F) -> Result<Payload<Activity>>
    where
        F: FnOnce(Activity) -> Activity,
    {
        self.execute(Command::SetActivity, SetActivityArgs::new(f), None)
    }

    /// Clear the users current activity
    pub fn clear_activity(&mut self) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::default(), None)
    }

    // NOTE: Not sure what the actual response values of
    //       SEND_ACTIVITY_JOIN_INVITE and CLOSE_ACTIVITY_REQUEST are,
    //       they are not documented.
    /// Send an invite to a user to join a game
    pub fn send_activity_join_invite(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::SendActivityJoinInvite,
            SendActivityJoinInviteArgs::new(user_id),
            None,
        )
    }

    /// Close request to join a game
    pub fn close_activity_request(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::CloseActivityRequest,
            CloseActivityRequestArgs::new(user_id),
            None,
        )
    }

    /// Subscribe to a given event
    pub fn subscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
    where
        F: FnOnce(SubscriptionArgs) -> SubscriptionArgs,
    {
        self.execute(Command::Subscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    /// Unsubscribe from a given event
    pub fn unsubscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
    where
        F: FnOnce(SubscriptionArgs) -> SubscriptionArgs,
    {
        self.execute(Command::Unsubscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    /// Register a handler for a given event
    pub fn on_event<F>(&mut self, event: Event, handler: F)
    where
        F: Fn(EventContext) + 'static + Send + Sync,
    {
        self.event_handler_registry.register(event, handler);
    }

    event_handler_function!(on_ready, Event::Ready);

    event_handler_function!(on_error, Event::Error);

    event_handler_function!(on_activity_join, Event::ActivityJoin);

    event_handler_function!(on_activity_join_request, Event::ActivityJoinRequest);

    event_handler_function!(on_activity_spectate, Event::ActivitySpectate);
}
