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
use crate::base::{DynamicAttributes, GameObjectAttributes};

const RESTITUTION: f32 = 0.5;

pub trait Collidable {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
    fn borrow_mut_dynamic_attributes(&mut self) -> &mut DynamicAttributes;
    fn borrow_mut_base_object(&mut self) -> &mut GameObjectAttributes;
    fn borrow_base_object(&self) -> &GameObjectAttributes;
    fn borrow_dynamic_attributes(&self) -> &DynamicAttributes;
    fn get_collidable_type(&self) -> u32;
    fn process_collision(&mut self, other: &dyn Collidable);

    fn is_collides(&self, other: &dyn Collidable) -> bool {
        let (self_x, self_y, self_width, self_height) = (
            self.get_x(),
            self.get_y(),
            self.get_width(),
            self.get_height(),
        );

        let (other_x, other_y, other_width, other_height) = (
            other.get_x(),
            other.get_y(),
            other.get_width(),
            other.get_height(),
        );

        self_x < other_x + other_width
            && self_x + self_width > other_x
            && self_y < other_y + other_height
            && self_y + self_height > other_y
    }

    fn compensate_overlap(&mut self, other: &dyn Collidable) {
        let self_obj = self.borrow_mut_base_object();
        let other_obj = other.borrow_base_object();

        let overlap_x = (self_obj.x + self_obj.width - other_obj.x)
            .min(other_obj.x + other_obj.width - self_obj.x);
        let overlap_y = (self_obj.y + self_obj.height - other_obj.y)
            .min(other_obj.y + other_obj.height - self_obj.y);

        if overlap_x < overlap_y {
            // Resolve in the x direction
            if self_obj.x < other_obj.x {
                self_obj.x -= overlap_x; // Move self left
            } else {
                self_obj.x += overlap_x; // Move self right
            }
        } else {
            // Resolve in the y direction
            if self_obj.y < other_obj.y {
                self_obj.y -= overlap_y; // Move self up
            } else {
                self_obj.y += overlap_y; // Move self down
            }
        }
    }

    fn change_dynamics(&mut self, other: &dyn Collidable) {
        let self_obj = self.borrow_base_object();
        let other_obj = other.borrow_base_object();
        // Calculate the collision normal
        let collision_normal = {
            let dx = (self_obj.x + self_obj.width / 2.0) - (other_obj.x + other_obj.width / 2.0);
            let dy = (self_obj.y + self_obj.height / 2.0) - (other_obj.y + other_obj.height / 2.0);
            (dx, dy)
        };

        // Normalize the collision normal
        let length = (collision_normal.0.powi(2) + collision_normal.1.powi(2)).sqrt();
        let (norm_x, norm_y) = if length != 0.0 {
            (collision_normal.0 / length, collision_normal.1 / length)
        } else {
            (0.0, 0.0) // Prevent division by zero
        };

        let self_dyn = self.borrow_mut_dynamic_attributes();
        // Calculate the self's velocity
        let self_velocity_x = self_dyn.vel_x;
        let self_velocity_y = self_dyn.vel_y;

        // Calculate the relative velocity of self along the collision normal
        let velocity_along_normal = self_velocity_x * norm_x + self_velocity_y * norm_y;

        // Only resolve if the object is moving towards the other object
        if velocity_along_normal > 0.0 {
            return; // They are separating
        }

        // Coefficient of restitution (bounciness)
        let restitution = RESTITUTION; // Adjust this value as needed

        // Calculate impulse scalar
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;

        // Apply impulse to self's velocity
        self_dyn.vel_x += impulse_scalar * norm_x;
        self_dyn.vel_y += impulse_scalar * norm_y;
    }

    fn try_collide(&mut self, other: &dyn Collidable) {
        if self.is_collides(other) {
            self.process_collision(other);
        }
    }
}
