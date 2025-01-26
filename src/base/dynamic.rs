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
use super::GameObjectAttributes;

pub trait Dynamic {
    fn set_velocity(&mut self, velocity_xy: (f32, f32));
    fn update_position(&mut self, delta_time: f32);
    fn set_friction(&mut self, friction: f32);
    fn apply_force(&mut self, force: (f32, f32));
}

pub struct DynamicAttributes {
    pub vel_x: f32,
    pub vel_y: f32,
    pub mass: f32,
    pub friction: f32,
    fx: f32,
    fy: f32,
    f_mult: f32,
}

impl DynamicAttributes {
    pub const MAX_FORCE: f32 = 100.0;
    const F_MULT: f32 = 1.0;

    pub fn new(mass: f32, friction_0_1: f32) -> Self {
        Self {
            vel_x: 0.0,
            vel_y: 0.0,
            mass,
            friction: friction_0_1,
            fx: 0.0,
            fy: 0.0,
            f_mult: DynamicAttributes::F_MULT,
        }
    }

    pub fn set_velocity(&mut self, velocity_xy: (f32, f32)) {
        self.vel_x = velocity_xy.0;
        self.vel_y = velocity_xy.1;
    }

    /// Set friction value. The value should be in the range [0.0, 1.0]
    pub fn set_friction(&mut self, friction: f32) {
        if friction < 0.0 {
            self.friction = 0.0;
        } else if friction > 1.0 {
            self.friction = 1.0;
        } else {
            self.friction = friction;
        }
    }

    pub fn apply_force(&mut self, force: (f32, f32)) {
        let mut fx_adjusted = self.f_mult * force.0;
        let mut fy_adjusted = self.f_mult * force.1;

        // Limit the force to the maximum value
        let magnitude = (force.0.powi(2) + force.1.powi(2)).sqrt();
        if magnitude > DynamicAttributes::MAX_FORCE {
            fx_adjusted *= DynamicAttributes::MAX_FORCE / magnitude;
            fy_adjusted *= DynamicAttributes::MAX_FORCE / magnitude;
        }

        self.fx = fx_adjusted;
        self.fy = fy_adjusted;
    }

    pub fn update_position(&mut self, base: &mut GameObjectAttributes, delta_time: f32) {
        let (x, y) = base.get_position();

        let acceleration_x = self.fx / self.mass;
        let acceleration_y = self.fy / self.mass;

        self.vel_x += acceleration_x;
        self.vel_y += acceleration_y;

        self.vel_x *= 1.0 - self.friction;
        self.vel_y *= 1.0 - self.friction;

        base.set_position((x + self.vel_x * delta_time, y + self.vel_y * delta_time));
    }
}
