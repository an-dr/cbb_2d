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
use macroquad::{color::*, shapes::draw_rectangle};
use rand::Rng;
use std::{any::Any, ops::Range};

pub trait GameObject: Any {
    fn get_mut_object_attr(&mut self) -> &mut GameObjectAttributes;
    fn get_object_attr(&self) -> &GameObjectAttributes;

    fn get_position(&self) -> (f32, f32);
    fn set_position(&mut self, position_xy: (f32, f32));

    fn get_id(&self) -> u32;
    fn set_id(&mut self, id: u32);

    fn get_name(&self) -> String;
    fn deletion_callback(&self);

    fn draw(&self, scale: f32);
}

pub struct GameObjectAttributes {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    request_deletion: bool,
}

impl GameObjectAttributes {
    pub fn new(id: u32, position_xy: (f32, f32), size_wh: (f32, f32)) -> Self {
        Self {
            id,
            x: position_xy.0,
            y: position_xy.1,
            width: size_wh.0,
            height: size_wh.1,
            request_deletion: false,
        }
    }

    pub fn random(
        id: u32,
        position_xy: (Range<f32>, Range<f32>),
        size_wh: (Range<f32>, Range<f32>),
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            x: rng.gen_range(position_xy.0),
            y: rng.gen_range(position_xy.1),
            width: rng.gen_range(size_wh.0),
            height: rng.gen_range(size_wh.1),
            request_deletion: false,
        }
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn set_position(&mut self, position_xy: (f32, f32)) {
        self.x = position_xy.0;
        self.y = position_xy.1;
    }

    pub fn request_deletion(&mut self) {
        self.request_deletion = true;
    }

    pub fn is_deletion_requested(&self) -> bool {
        self.request_deletion
    }
}
