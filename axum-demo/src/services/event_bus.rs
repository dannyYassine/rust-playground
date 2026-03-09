use std::any::{Any, TypeId};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct EventBus {
    listeners: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn listen<E>(&mut self, callback: impl Fn(&E) + 'static)
    where
        E: 'static + Send,
    {
        let type_id = TypeId::of::<E>();
        let boxed: Box<dyn Any> = Box::new(Box::new(callback) as Box<dyn Fn(&E)>);
        self.listeners.entry(type_id).or_default().push(boxed);
    }

    #[allow(dead_code)]
    pub fn emit<E>(&self, event: &E)
    where
        E: 'static + Send,
    {
        if let Some(listeners) = self.listeners.get(&TypeId::of::<E>()) {
            for listener in listeners {
                if let Some(callback) = listener.downcast_ref::<Box<dyn Fn(&E)>>() {
                    callback(event);
                }
            }
        }
    }
}

// let event = Event {
//     data: "Hello, EventBus!".to_string(),
// };
// let mut event_bus = EventBus::new();
// event_bus.listen::<Event>(|event: &Event| {
//     println!("Event received: {:?}", event);
//     println!("Event received: {:?}", event);
// });
// event_bus.emit(&event);
