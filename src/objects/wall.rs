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
    color::{GRAY, WHITE},
    shapes::draw_rectangle,
};

use super::CollidableType;
use crate::{
    base::{GameObject, GameObjectAttributes},
    sys_collision::Collidable,
};
use std::ops::Range;

pub struct Wall {
    obj_attr: GameObjectAttributes,
}

impl Wall {
    pub fn new(id: u32, position_xy: (f32, f32), size_wh: (f32, f32)) -> Self {
        Self {
            obj_attr: GameObjectAttributes::new(id, position_xy, size_wh),
        }
    }

    pub fn random(
        id: u32,
        position_xy: (Range<f32>, Range<f32>),
        size_wh: (Range<f32>, Range<f32>),
    ) -> Self {
        Self {
            obj_attr: GameObjectAttributes::random(id, position_xy, size_wh),
        }
    }
}

impl GameObject for Wall {
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

    fn get_id(&self) -> u32 {
        self.obj_attr.id
    }

    fn set_id(&mut self, id: u32) {
        self.obj_attr.id = id;
    }

    fn draw(&self, _scale: f32) {
        let color = GRAY;
        let (x, y) = self.get_position();
        let (w, h) = (self.obj_attr.width, self.obj_attr.height);
        draw_rectangle(x * _scale, y * _scale, w * _scale, h * _scale, color);
    }

    fn get_name(&self) -> String {
        format!("Wall loc[{},{}]", self.obj_attr.x, self.obj_attr.y)
    }

    fn deletion_callback(&self) {
        // Do nothing
    }
}

impl Collidable for Wall {
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
        CollidableType::SolidObstacle.to_int()
    }

    fn process_collision(&mut self, _other: &dyn Collidable) {
        // Do nothing
    }
}
