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
use super::{CollidableType, PlayerMode};
use crate::{
    base::{Dynamic, DynamicAttributes, GameObject, GameObjectAttributes},
    sys_collision::Collidable,
    sys_event::{Event, Subscriber},
};
use macroquad::prelude::*;
use std::{clone, fmt};

pub struct Player {
    pub player_id: u32,
    player_mode: PlayerMode,
    dynamic_data: DynamicAttributes,
    dynamic_data_default: DynamicAttributes,
    obj_attr: GameObjectAttributes,
    pub score: u32,
    pub color_default: Color,
    pub color_current: Color,
}

impl Player {
    pub fn new(
        player_id: u32,
        obj_id: u32,
        position_xy: (f32, f32),
        size_wh: (f32, f32),
        mass: f32,
        friction: f32,
        color: Color,
    ) -> Self {
        // Create a new player object and subscribe to the event bus
        let player = Self {
            player_id,
            player_mode: PlayerMode::Normal,
            dynamic_data: DynamicAttributes::new(mass, friction),
            dynamic_data_default: DynamicAttributes::new(mass, friction),
            obj_attr: GameObjectAttributes::new(obj_id, position_xy, size_wh),
            score: 0,
            color_default: color,
            color_current: color,
        };
        player
    }

    pub fn set_mode(&mut self, mode: PlayerMode) {
        match mode {
            PlayerMode::Normal => {
                self.color_current = self.color_default;
                self.set_friction(self.dynamic_data_default.friction);
                self.dynamic_data.mass = self.dynamic_data_default.mass;
            }
            PlayerMode::NoClip => {
                self.color_current = WHITE;
                self.set_friction(0.7);
                self.dynamic_data.mass = 1.4;
            }
        }
        self.player_mode = mode;
    }
}

impl GameObject for Player {
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
        format!("Player no {}", self.player_id)
    }

    fn draw(&self, _scale: f32) {
        draw_rectangle(
            self.obj_attr.x * _scale,
            self.obj_attr.y * _scale,
            self.obj_attr.width * _scale,
            self.obj_attr.height * _scale,
            self.color_current,
        );
    }

    fn deletion_callback(&self) {
        log::debug!("Player {} deleted", self.player_id);
    }
}

impl Dynamic for Player {
    fn set_velocity(&mut self, velocity_xy: (f32, f32)) {
        self.dynamic_data.set_velocity(velocity_xy);
    }

    fn update_position(&mut self, delta_time: f32) {
        self.dynamic_data
            .update_position(&mut self.obj_attr, delta_time);
    }

    fn set_friction(&mut self, friction: f32) {
        self.dynamic_data.set_friction(friction);
    }

    fn apply_force(&mut self, force: (f32, f32)) {
        self.dynamic_data.apply_force(force);
    }
}

impl Collidable for Player {
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

    fn borrow_mut_dynamic_attributes(&mut self) -> &mut DynamicAttributes {
        &mut self.dynamic_data
    }

    fn borrow_dynamic_attributes(&self) -> &DynamicAttributes {
        &self.dynamic_data
    }

    fn borrow_base_object(&self) -> &GameObjectAttributes {
        &self.obj_attr
    }

    fn borrow_mut_base_object(&mut self) -> &mut GameObjectAttributes {
        &mut self.obj_attr
    }

    fn process_collision(&mut self, other: &dyn Collidable) {
        let other_type = CollidableType::from_int(other.get_collidable_type());

        match other_type {
            CollidableType::SolidObstacle => match self.player_mode {
                PlayerMode::Normal => {
                    self.compensate_overlap(other);
                    self.change_dynamics(other);
                }
                PlayerMode::NoClip => {}
            },
            CollidableType::Collectable => {
                self.score += 1;
                log::debug!("Player {} score: {}", self.player_id, self.score);
            }
            _ => {}
        }
    }

    fn get_collidable_type(&self) -> u32 {
        CollidableType::Player.to_int()
    }
}

impl Subscriber for Player {
    fn handle_busevent(&mut self, event: &Event) {
        match event {
            Event::MovePlayer { id, fx, fy } => {
                if *id == self.player_id {
                    self.apply_force((*fx, *fy));
                }
            }
            Event::ChangeColor { id, color } => {
                if *id == self.player_id {
                    match color {
                        None => self.color_current = self.color_default,
                        Some(color) => self.color_current = *color,
                    }
                }
            }
            Event::SetPlayerMode { id, mode } => {
                if *id == self.player_id {
                    self.set_mode(mode.clone());
                }
            }
            _ => {}
        }
    }
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player no {}", self.player_id)
    }
}
