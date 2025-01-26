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
use super::event::Event;
use super::sys_event::SysEvent;
use std::sync::{Arc, Mutex};

pub trait Publisher {
    fn get_event_bus(&self) -> Arc<Mutex<SysEvent>>;
    fn publish(&self, event: Event) {
        let bus = self.get_event_bus();
        bus.lock().unwrap().publish(event);
    }
}
