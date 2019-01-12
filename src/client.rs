use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use crate::{
    connection::Manager as ConnectionManager,
    models::{
        OpCode,
        Command,
        Event,
        payload::Payload,
        message::Message,
        commands::{SubscriptionArgs, Subscription},
    },
    Error,
    Result,
    event_handler::{
        HandlerRegistry,
        Context as EventContext,
    },
};
#[cfg(feature = "rich_presence")]
use crate::models::rich_presence::{
    SetActivityArgs,
    Activity,
    SendActivityJoinInviteArgs,
    CloseActivityRequestArgs,
};


#[derive(Clone)]
pub struct Client {
    connection_manager: ConnectionManager,
    event_handler_registry: HandlerRegistry<'static>,
}

impl Client {
    pub fn new(client_id: u64) -> Self {
        let event_handler_registry = HandlerRegistry::new();
        let connection_manager = ConnectionManager::new(client_id, event_handler_registry.clone());
        Self { connection_manager, event_handler_registry }
    }

    pub fn start(&mut self) {
        self.connection_manager.start();
    }

    fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
        where A: Serialize + Send + Sync,
              E: Serialize + DeserializeOwned + Send + Sync
    {
        let message = Message::new(OpCode::Frame, Payload::with_nonce(cmd, Some(args), None, evt));
        self.connection_manager.send(message)?;
        let Message { payload, .. } = self.connection_manager.recv()?;
        let response: Payload<E> = serde_json::from_str(&payload)?;

        match response.evt {
            Some(Event::Error) => Err(Error::SubscriptionFailed),
            _ => Ok(response)
        }
    }

    #[cfg(feature = "rich_presence")]
    pub fn set_activity<F>(&mut self, f: F) -> Result<Payload<Activity>>
        where F: FnOnce(Activity) -> Activity
    {
        self.execute(Command::SetActivity, SetActivityArgs::new(f), None)
    }

    #[cfg(feature = "rich_presence")]
    pub fn clear_activity(&mut self) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::default(), None)
    }

    // NOTE: Not sure what the actual response values of
    //       SEND_ACTIVITY_JOIN_INVITE and CLOSE_ACTIVITY_REQUEST are,
    //       they are not documented.
    #[cfg(feature = "rich_presence")]
    pub fn send_activity_join_invite(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(Command::SendActivityJoinInvite, SendActivityJoinInviteArgs::new(user_id), None)
    }

    #[cfg(feature = "rich_presence")]
    pub fn close_activity_request(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(Command::CloseActivityRequest, CloseActivityRequestArgs::new(user_id), None)
    }

    pub fn subscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
        where F: FnOnce(SubscriptionArgs) -> SubscriptionArgs
    {
        self.execute(Command::Subscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    pub fn unsubscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
        where F: FnOnce(SubscriptionArgs) -> SubscriptionArgs
    {
        self.execute(Command::Unsubscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    pub fn on_ready<F>(&mut self, handler: F)
        where F: Fn(EventContext) + 'static + Send + Sync
    {
        self.event_handler_registry.register(Event::Ready, handler);
    }
}
