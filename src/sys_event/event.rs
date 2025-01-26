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
use macroquad::color::Color;
use crate::objects::PlayerMode;

#[derive(Debug)]
pub enum Event {
    MovePlayer { id: u32, fx: f32, fy: f32 },
    SetPlayerMode { id: u32, mode: PlayerMode },
    ChangeColor { id: u32, color: Option<Color> },
    Quit,
}
