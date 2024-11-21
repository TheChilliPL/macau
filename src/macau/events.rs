use crate::macau::{MacauEvent, MacauGame};

pub struct EventManager {
    subscribers: Vec<(u32, Box<dyn Fn(&MacauGame, &MacauEvent)>)>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, player_id: u32, subscriber: F)
    where
        F: Fn(&MacauGame, &MacauEvent) + 'static,
    {
        self.subscribers.push((player_id, Box::new(subscriber)));
    }

    pub fn notify_common(&self, game: &MacauGame, event: &MacauEvent) {
        for (_, subscriber) in &self.subscribers {
            subscriber(game, event);
        }
    }

    pub fn notify_customized<F>(&self, game: &MacauGame, func: F)
    where
        F: Fn(&MacauGame, u32) -> MacauEvent,
    {
        for (id, subscriber) in &self.subscribers {
            let event = func(game, *id);
            subscriber(game, &event)
        }
    }
}
