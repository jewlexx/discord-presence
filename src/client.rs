use std::{
    sync::atomic::Ordering,
    thread::{JoinHandle, Thread},
};

use crate::{
    connection::Manager as ConnectionManager,
    event_handler::{Context as EventContext, HandlerRegistry},
    models::{
        commands::{Subscription, SubscriptionArgs},
        message::Message,
        payload::Payload,
        rich_presence::{
            Activity, CloseActivityRequestArgs, SendActivityJoinInviteArgs, SetActivityArgs,
        },
        Command, Event, OpCode,
    },
    DiscordError, Result,
};
use crossbeam_channel::Sender;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

macro_rules! event_handler_function {
    ( $( $name:ident, $event:expr ),* ) => {
        event_handler_function!{@gen $([ $name, $event])*}
    };

    (@gen $( [ $name:ident, $event:expr ] ), *) => {
        $(
            #[doc = concat!("Listens for the `", stringify!($event), "` event")]
            pub fn $name<F>(&mut self, handler: F)
                where F: Fn(EventContext) + 'static + Send + Sync
            {
                self.on_event($event, handler);
            }
        )*
    }
}

/// Wrapper around the [`JoinHandle`] returned by [`Client::start`]
#[allow(clippy::module_name_repetitions)]
pub struct ClientThread(JoinHandle<()>, Sender<()>);

impl ClientThread {
    // Ignore missing error docs because it's an alias of `join`
    #[allow(clippy::missing_errors_doc)]
    /// Alias of [`JoinHandle::join()`]
    pub fn join(self) -> std::thread::Result<()> {
        self.0.join()
    }

    // Ignore missing error docs because it's an alias of `is_finished`
    #[allow(clippy::missing_errors_doc)]
    #[must_use]
    /// Alias of [`JoinHandle::is_finished`]
    pub fn is_finished(&self) -> bool {
        self.0.is_finished()
    }
    // Ignore missing error docs because it's an alias of `thread`
    #[allow(clippy::missing_errors_doc)]
    #[must_use]
    /// Alias of [`JoinHandle::thread`]
    pub fn thread(&self) -> &Thread {
        self.0.thread()
    }

    /// Attempt to stop the client's send and receive loop
    ///
    /// # Errors
    /// - Failed to send stop message (maybe it has already stopped?)
    /// - The event loop had its own error
    pub fn stop(self) -> Result<()> {
        // Attempt to send the message to stop the thread
        self.1.send(())?;

        self.join().map_err(|_| DiscordError::EventLoopError)?;

        Ok(())
    }

    /// "Forgets" client thread, removing the variable, but keeping the client running indefinitely.
    pub fn persist(self) {
        std::mem::forget(self);
    }
}

/// The Discord client
#[derive(Clone)]
pub struct Client {
    connection_manager: ConnectionManager,
    event_handler_registry: HandlerRegistry<'static>,
}

#[cfg(feature = "bevy")]
impl bevy::ecs::system::Resource for Client {}

impl Client {
    /// Creates a new `Client`
    #[must_use]
    pub fn new(client_id: u64) -> Self {
        let event_handler_registry = HandlerRegistry::new();
        Self {
            connection_manager: ConnectionManager::new(client_id, event_handler_registry.clone()),
            event_handler_registry,
        }
    }

    // TODO: Add examples
    /// Start the connection manager
    ///
    /// Only join the thread if there is no other task keeping the program alive.
    ///
    /// This must be called before all and any actions such as `set_activity`
    #[must_use = "the client will be immediately dropped if the handle is not kept"]
    pub fn start(&mut self) -> ClientThread {
        let (tx, rx) = crossbeam_channel::bounded::<()>(1);

        let thread = self.connection_manager.start(rx);

        crate::STARTED.store(true, Ordering::Relaxed);

        self.on_ready(|_| {
            trace!("Discord client is ready!");
            crate::READY.store(true, Ordering::Relaxed);
        });

        ClientThread(thread, tx)
    }

    /// Check if the client is ready
    pub fn is_ready() -> bool {
        crate::READY.load(Ordering::Acquire)
    }

    /// Check if the client has started
    pub fn is_started() -> bool {
        crate::STARTED.load(Ordering::Acquire)
    }

    fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
    where
        A: Serialize + Send + Sync,
        E: Serialize + DeserializeOwned + Send + Sync,
    {
        if !crate::STARTED.load(Ordering::Relaxed) || !crate::READY.load(Ordering::Relaxed) {
            return Err(DiscordError::NotStarted);
        }

        trace!("Executing command: {:?}", cmd);

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
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
    pub fn set_activity<F>(&mut self, f: F) -> Result<Payload<Activity>>
    where
        F: FnOnce(Activity) -> Activity,
    {
        self.execute(Command::SetActivity, SetActivityArgs::new(f), None)
    }

    /// Clear the users current activity
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
    pub fn clear_activity(&mut self) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::default(), None)
    }

    // NOTE: Not sure what the actual response values of
    //       SEND_ACTIVITY_JOIN_INVITE and CLOSE_ACTIVITY_REQUEST are,
    //       they are not documented.
    /// Send an invite to a user to join a game
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
    pub fn send_activity_join_invite(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::SendActivityJoinInvite,
            SendActivityJoinInviteArgs::new(user_id),
            None,
        )
    }

    /// Close request to join a game
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
    pub fn close_activity_request(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(
            Command::CloseActivityRequest,
            CloseActivityRequestArgs::new(user_id),
            None,
        )
    }

    /// Subscribe to a given event
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
    pub fn subscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
    where
        F: FnOnce(SubscriptionArgs) -> SubscriptionArgs,
    {
        self.execute(Command::Subscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    /// Unsubscribe from a given event
    ///
    /// # Errors
    /// - See [`DiscordError`] for more info
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

    /// Block the current thread until the event is fired
    ///
    /// Returns the context the event was fired in
    ///
    /// NOTE: Please only use this for the ready event, or if you know what you are doing.
    ///
    /// # Errors
    /// - Channel disconnected
    ///
    /// # Panics
    /// - Panics if the channel is disconnected for whatever reason.
    pub fn block_until_event(&mut self, event: Event) -> Result<crate::event_handler::Context> {
        let (tx, rx) = crossbeam_channel::bounded::<crate::event_handler::Context>(1);

        let handler = move |info| {
            if let Err(e) = tx.send(info) {
                error!("{e}");
            }
        };

        self.event_handler_registry.register(event, handler);

        Ok(rx.recv()?)
    }

    event_handler_function!(on_ready, Event::Ready);

    event_handler_function!(on_error, Event::Error);

    event_handler_function!(on_activity_join, Event::ActivityJoin);

    event_handler_function!(on_activity_join_request, Event::ActivityJoinRequest);

    event_handler_function!(on_activity_spectate, Event::ActivitySpectate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_started() {
        assert!(!Client::is_started());

        crate::STARTED.store(true, Ordering::Relaxed);

        assert!(Client::is_started());
    }

    #[test]
    fn test_is_ready() {
        assert!(!Client::is_ready());

        crate::READY.store(true, Ordering::Relaxed);

        assert!(Client::is_ready());
    }
}
