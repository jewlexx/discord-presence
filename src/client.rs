use std::{sync::atomic::Ordering, time::Instant};

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
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

macro_rules! event_handler_function {
    ( $( $name:ident, $event:expr ),* ) => {
        event_handler_function!{@gen $([ $name, $event])*}
    };

    (@gen $( [ $name:ident, $event:expr ] ), *) => {
        ::paste::paste! {
            $(
                #[doc = concat!("Listens for the `", stringify!($event), "` event, and runs the provided handler")]
                pub fn [<on_ $name>]<F>(&mut self, handler: F)
                    where F: Fn(EventContext) + 'static + Send + Sync
                {
                    self.on_event($event, handler);
                }

                #[doc = concat!("Blocks the current thread until the `", stringify!($event), "` event is fired")]
                pub fn [<block_until_ $name>](&mut self) -> Result<crate::event_handler::Context> {
                    self.block_until_event($event)
                }
            )*
        }
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

    /// Start the connection manager
    ///
    /// Only join the thread if there is no other task keeping the program alive.
    ///
    /// This must be called before all and any actions such as `set_activity`
    #[must_use]
    pub fn start(&mut self) -> std::thread::JoinHandle<()> {
        let thread = self.connection_manager.start();

        crate::STARTED.store(true, Ordering::Relaxed);

        self.on_ready(|_| {
            trace!("Discord client is ready!");
            crate::READY.store(true, Ordering::Relaxed);
        });

        thread
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
    pub fn set_activity<F>(&mut self, f: F) -> Result<Payload<Activity>>
    where
        F: FnOnce(Activity) -> Activity,
    {
        {
            let old_rl = crate::RATE_LIMIT_CHECK.lock();
            if old_rl.0.elapsed().as_secs() < 20 {
                if old_rl.1 > 5 {
                    return Err(DiscordError::RateLimited);
                }
            } else {
                *crate::RATE_LIMIT_CHECK.lock() = (Instant::now(), 0);
            }
        }

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

    /// Block the current thread until the event is fired
    ///
    /// Returns the context the event was fired in
    ///
    /// NOTE: Please only use this for the ready event, or if you know what you are doing.
    ///
    /// # Panics
    ///
    /// Panics if the channel is disconnected for whatever reason.
    pub fn block_until_event(&mut self, event: Event) -> Result<crate::event_handler::Context> {
        let (tx, rx) = crossbeam_channel::bounded::<crate::event_handler::Context>(1);

        let handler = move |info| tx.send(info).unwrap();

        self.event_handler_registry.register(event, handler);

        Ok(rx.recv()?)
    }

    event_handler_function!(ready, Event::Ready);

    event_handler_function!(error, Event::Error);

    event_handler_function!(activity_join, Event::ActivityJoin);

    event_handler_function!(activity_join_request, Event::ActivityJoinRequest);

    event_handler_function!(activity_spectate, Event::ActivitySpectate);
}
