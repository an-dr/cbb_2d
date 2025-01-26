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

use std::{clone, fmt};

pub enum PlayerMode {
    Normal,
    NoClip,
}

impl fmt::Debug for PlayerMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerMode::Normal => write!(f, "Normal"),
            PlayerMode::NoClip => write!(f, "NoClip"),
        }
    }
}

impl clone::Clone for PlayerMode {
    fn clone(&self) -> Self {
        match self {
            PlayerMode::Normal => PlayerMode::Normal,
            PlayerMode::NoClip => PlayerMode::NoClip,
        }
    }
}
