use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{client::EVENT_HANDLER_REGISTRY, models::Event, Result};

type Handler<'a> = (Box<dyn Fn(Context) + 'a + Send + Sync>, Uuid);

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

pub struct EventHandle {
    pub event: Event,
    pub uuid: Uuid,
}

impl Drop for EventHandle {
    fn drop(&mut self) {
        EVENT_HANDLER_REGISTRY.unregister(self.uuid).unwrap();
    }
}

#[derive(Clone)]
pub struct HandlerRegistry<'a> {
    handlers: Arc<RwLock<HashMap<Event, HandlerList<'a>>>>,
}

impl<'a> HandlerRegistry<'a> {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<F>(&self, event: Event, handler: F) -> Uuid
    where
        F: Fn(Context) + 'a + Send + Sync,
    {
        let uuid = Uuid::new_v4();
        let mut event_handlers = self.handlers.write();
        let event_handler = event_handlers.entry(event).or_default();
        event_handler.push((Box::new(handler), uuid));

        uuid
    }

    pub fn handle(&self, event: Event, data: JsonValue) -> Result<()> {
        let handlers = self.handlers.read();
        if let Some(handlers) = handlers.get(&event) {
            let context = Context::new(data);

            for handler in handlers {
                handler.0(context.clone())
            }
        }

        Ok(())
    }

    pub fn unregister_events(&self, event: Event) -> Result<()> {
        let mut handlers = self.handlers.write();
        handlers.remove(&event);

        Ok(())
    }

    pub fn unregister(&self, uuid: Uuid) -> Result<()> {
        let mut handlers = self.handlers.write();
        for (_, handlers) in handlers.iter_mut() {
            handlers.retain(|handler| handler.1 != uuid);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_register_event_handlers() {
        let registry = HandlerRegistry::new();
        registry.register(Event::Ready, |_| unimplemented!());
        registry.register(Event::Ready, |_| unimplemented!());
        registry.register(Event::Error, |_| unimplemented!());

        let handlers = registry.handlers.read();
        assert_eq!(handlers.len(), 2);
        assert_eq!(handlers[&Event::Ready].len(), 2);
        assert_eq!(handlers[&Event::Error].len(), 1);
    }
}
