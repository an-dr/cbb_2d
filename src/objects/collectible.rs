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

use std::ops::Range;

use super::CollidableType;
use crate::{
    base::{GameObject, GameObjectAttributes},
    sys_collision::Collidable,
};
use macroquad::{color::GREEN, shapes::draw_rectangle};

pub struct Collectable {
    obj_attr: GameObjectAttributes,
}

static COLLECTIBLE_SIZE: f32 = 10.0;

impl Collectable {
    pub fn new(x: f32, y: f32) -> Collectable {
        Collectable {
            obj_attr: GameObjectAttributes::new(0, (x, y), (COLLECTIBLE_SIZE, COLLECTIBLE_SIZE)),
        }
    }

    pub fn random(position_xy: (Range<f32>, Range<f32>)) -> Collectable {
        Collectable {
            obj_attr: GameObjectAttributes::random(
                0,
                (position_xy.0, position_xy.1),
                (
                    COLLECTIBLE_SIZE..COLLECTIBLE_SIZE + 1.0,
                    COLLECTIBLE_SIZE..COLLECTIBLE_SIZE + 1.0,
                ),
            ),
        }
    }
}

impl GameObject for Collectable {
    fn get_id(&self) -> u32 {
        self.obj_attr.id
    }

    fn set_id(&mut self, id: u32) {
        self.obj_attr.id = id;
    }

    fn get_mut_object_attr(&mut self) -> &mut GameObjectAttributes {
        &mut self.obj_attr
    }

    fn get_object_attr(&self) -> &GameObjectAttributes {
        &self.obj_attr
    }

    fn get_position(&self) -> (f32, f32) {
        (self.obj_attr.x, self.obj_attr.y)
    }

    fn set_position(&mut self, position_xy: (f32, f32)) {
        self.obj_attr.x = position_xy.0;
        self.obj_attr.y = position_xy.1;
    }

    fn get_name(&self) -> String {
        format!("Collectable loc[{},{}]", self.obj_attr.x, self.obj_attr.y)
    }

    fn draw(&self, _scale: f32) {
        draw_rectangle(
            self.obj_attr.x * _scale,
            self.obj_attr.y * _scale,
            self.obj_attr.width * _scale,
            self.obj_attr.height * _scale,
            GREEN,
        );
    }
    
    fn deletion_callback(&self) {
        // Do nothing
    }
}

impl Collidable for Collectable {
    fn get_x(&self) -> f32 {
        self.obj_attr.x
    }

    fn get_y(&self) -> f32 {
        self.obj_attr.y
    }

    fn get_width(&self) -> f32 {
        self.obj_attr.width
    }

    fn get_height(&self) -> f32 {
        self.obj_attr.height
    }

    fn borrow_mut_dynamic_attributes(&mut self) -> &mut crate::base::DynamicAttributes {
        unimplemented!()
    }

    fn borrow_dynamic_attributes(&self) -> &crate::base::DynamicAttributes {
        unimplemented!()
    }

    fn borrow_mut_base_object(&mut self) -> &mut crate::base::GameObjectAttributes {
        &mut self.obj_attr
    }

    fn borrow_base_object(&self) -> &crate::base::GameObjectAttributes {
        &self.obj_attr
    }

    fn get_collidable_type(&self) -> u32 {
        CollidableType::Collectable.to_int()
    }

    fn process_collision(&mut self, other: &dyn Collidable) {
        // Do nothing
        if other.get_collidable_type() == CollidableType::Player.to_int() {
            self.borrow_mut_base_object().request_deletion();
        }
    }
}
