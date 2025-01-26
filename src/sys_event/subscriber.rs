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
use std::fmt;

pub trait Subscriber {
    fn handle_busevent(&mut self, event: &Event);
}

impl fmt::Debug for dyn Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implementing Debug manually for trait objects
        write!(f, "Subscriber no ?")
    }
}
