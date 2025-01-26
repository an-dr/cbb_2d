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

use macroquad::{
    color::{RED, WHITE},
    text::{draw_text_ex, TextParams},
};

pub struct Osd {
    text: String,
}

impl Osd {
    pub fn new() -> Self {
        Self {
            text: "OSD Placeholder".to_string(),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn draw(&self, scale: f32) {
        draw_text_ex(
            &self.text,
            30.0,
            50.0,
            TextParams {
                font_size: 30,
                color: WHITE,
                font_scale: scale,
                ..Default::default()
            },
        );
    }
}
