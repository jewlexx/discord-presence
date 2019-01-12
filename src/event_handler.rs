use std::{
    collections::HashMap,
    sync::Arc,
};
use parking_lot::RwLock;
use serde_json::Value as JsonValue;
use crate::{
    models::Event,
    Result,
};


type Handler<'a> = Box<Fn(Context) + 'a + Send + Sync>;

type HandlerList<'a> = Vec<Handler<'a>>;


#[derive(Debug, Clone)]
pub struct Context {
    // TODO: implement event data structures
    pub event: JsonValue,
}

impl Context {
    pub fn new(event: JsonValue) -> Self {
        Self { event }
    }
}


#[derive(Clone)]
pub struct HandlerRegistry<'a> {
    handlers: Arc<RwLock<HashMap<Event, HandlerList<'a>>>>,
}

impl<'a> HandlerRegistry<'a> {
    pub fn new() -> Self {
        Self { handlers: Arc::new(RwLock::new(HashMap::new())) }
    }

    // TODO: return event index?
    pub fn register<F>(&mut self, event: Event, handler: F)
        where F: Fn(Context) + 'a + Send + Sync
    {
        let mut handlers = self.handlers.write();
        handlers.get_mut(&event).map(|inner| inner.push(Box::new(handler)));
    }

    pub fn unregister(&mut self, event: Event, id: usize) {
        let mut handlers = self.handlers.write();
        handlers.get_mut(&event).map(|inner| inner.remove(id));
    }

    pub fn handle(&mut self, event: Event, data: JsonValue) -> Result<()> {
        let handlers = self.handlers.read();
        if let Some(handlers) = handlers.get(&event) {
            let context = Context::new(data);

            for handler in handlers {
                handler(context.clone())
            }
        }

        Ok(())
    }
}
