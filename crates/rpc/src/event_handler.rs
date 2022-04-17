use crate::{models::Event, Result};
use parking_lot::RwLock;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Arc};

type Handler<'a> = Box<dyn Fn(Context) + 'a + Send + Sync>;

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
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<F>(&mut self, event: Event, handler: F)
    where
        F: Fn(Context) + 'a + Send + Sync,
    {
        let mut event_handlers = self.handlers.write();
        let event_handler = event_handlers.entry(event).or_default();
        event_handler.push(Box::new(handler));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_register_event_handlers() {
        let mut registry = HandlerRegistry::new();
        registry.register(Event::Ready, |_| unimplemented!());
        registry.register(Event::Ready, |_| unimplemented!());
        registry.register(Event::Error, |_| unimplemented!());

        let handlers = registry.handlers.read();
        assert_eq!(handlers.len(), 2);
        assert_eq!(handlers[&Event::Ready].len(), 2);
        assert_eq!(handlers[&Event::Error].len(), 1);
    }
}
