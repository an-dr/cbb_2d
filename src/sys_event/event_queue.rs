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
use super::Event;
use std::collections::VecDeque;

pub struct EventQueue {
    events: VecDeque<Event>,
}

impl EventQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_size),
        }
    }

    // Push a new event into the queue
    pub fn push(&mut self, event: Event) {
        self.events.push_back(event);
    }

    // Pop an event from the queue
    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}
