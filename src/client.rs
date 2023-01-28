use std::{
    mem::forget,
    sync::{atomic::Ordering, Arc},
};

use crate::{
    connection::Manager as ConnectionManager,
    event_handler::{Context as EventContext, EventCallbackHandle, HandlerRegistry},
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
        $(
            #[doc = concat!("Listens for the `", stringify!($event), "` event")]
            #[must_use = "event listeners will be immediately dropped if the handle is not kept"]
            pub fn $name<F>(&self, handler: F) -> EventCallbackHandle
                where F: FnMut(EventContext) + 'static + Send + Sync
            {
                self.on_event($event, handler)
            }
        )*
    }
}

/// The Discord client
#[derive(Clone)]
pub struct Client {
    connection_manager: ConnectionManager,
    event_handler_registry: Arc<HandlerRegistry>,
}

#[cfg(feature = "bevy")]
impl bevy::ecs::system::Resource for Client {}

impl Client {
    /// Creates a new `Client`
    pub fn new(client_id: u64) -> Self {
        let event_handler_registry = Arc::new(HandlerRegistry::new());
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

        let ready = self.on_ready(|_| {
            trace!("Discord client is ready!");
            crate::READY.store(true, Ordering::Relaxed);
        });

        forget(ready);

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

    /// Listens for a given event, and returns a handle that unregisters the listener when it is dropped.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::{thread::sleep, time::Duration};
    /// # use discord_presence::Client;
    /// let mut drpc = Client::new(1003450375732482138);
    /// let _ready = drpc.on_ready(|_ctx| {
    ///     println!("READY!");
    /// });
    ///
    /// let drpc_thread = drpc.start();
    ///
    /// {
    ///     let _ready_first_3_seconds = drpc.on_ready(|_ctx| {
    ///         println!("READY, IN THE FIRST 3 SECONDS!");
    ///     });
    ///     sleep(Duration::from_secs(3));
    /// }
    ///
    /// drpc_thread.join().unwrap()
    /// ```
    ///
    /// You can use [`std::mem::forget`] to disable the automatic unregister-on-drop:
    ///
    /// ```no_run
    /// # use discord_presence::Client;
    /// # let mut drpc = Client::new(1003450375732482138);
    ///
    /// {
    ///     let ready = drpc.on_ready(|_ctx| {
    ///         println!("READY!");
    ///     });
    ///     std::mem::forget(ready);
    /// }
    /// // the event listener is still registered
    ///
    /// # let drpc_thread = drpc.start();
    /// # drpc_thread.join().unwrap()
    /// ```
    #[must_use = "event listeners will be immediately dropped if the handle is not kept"]
    pub fn on_event<F>(&self, event: Event, handler: F) -> EventCallbackHandle
    where
        F: FnMut(EventContext) + 'static + Send + Sync,
    {
        self.event_handler_registry.register(event, handler)
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

        // `handler` is automatically unregistered once this variable drops
        let _cb_handle = self.on_event(event, handler);

        Ok(rx.recv()?)
    }

    event_handler_function!(on_ready, Event::Ready);

    event_handler_function!(on_error, Event::Error);

    event_handler_function!(on_activity_join, Event::ActivityJoin);

    event_handler_function!(on_activity_join_request, Event::ActivityJoinRequest);

    event_handler_function!(on_activity_spectate, Event::ActivitySpectate);
}
