// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************

use super::{event_queue::EventQueue, Event, Subscriber};
use log::*;
use std::sync::{Arc, Mutex};

pub struct SysEvent {
    queue: EventQueue,
    /// Thread safe list of subscribers
    subscribers: Arc<Mutex<Vec<Arc<Mutex<dyn Subscriber>>>>>,
}

impl SysEvent {
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(100),
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Use a shared reference (`&self`), since we're only modifying the inner `Vec`
    pub fn subscribe(&self, subscriber: Arc<Mutex<dyn Subscriber>>) {
        self.subscribers.lock().unwrap().push(subscriber);
    }

    // Method to publish an event to all subscribers
    pub fn publish(&mut self, event: Event) {
        self.queue.push(event);
    }

    // Method to process all events in the queue
    pub fn process_all(&mut self) {
        while let Some(event) = self.queue.pop() {
            self.process(event);
        }
    }

    fn process(&self, event: Event) {
        let mut subscribers = self.subscribers.lock().unwrap();
        for subscriber in subscribers.iter_mut() {
            subscriber.lock().unwrap().handle_busevent(&event);
        }
    }
}
