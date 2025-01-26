use crate::sys_event::SysEvent;

/// *************************************************************************
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
use super::Collidable;
use std::sync::{Arc, Mutex};

pub struct CollidableObjectWrap {
    object: Arc<Mutex<dyn Collidable>>,
}

pub struct SysCollision {
    objects: Vec<Arc<Mutex<dyn Collidable>>>,
    event_bus: Arc<Mutex<SysEvent>>,
}

impl SysCollision {
    pub fn new(event_bus: Arc<Mutex<SysEvent>>) -> Self {
        Self {
            objects: Vec::new(),
            event_bus,
        }
    }

    pub fn add_collidable_object(&mut self, object: Arc<Mutex<dyn Collidable>>) {
        log::debug!(
            "Adding collidable object: {:?}",
            object.lock().unwrap().borrow_base_object().id
        );
        self.objects.push(object);
    }

    pub fn add_static_object(&mut self, object: Arc<Mutex<dyn Collidable>>) {
        self.objects.push(object);
    }

    pub fn cleanup_objects(&mut self) {
        // Get IDs of objects to remove
        let mut ids_to_remove = Vec::new();
        for obj in &self.objects {
            if obj
                .lock()
                .unwrap()
                .borrow_base_object()
                .is_deletion_requested()
            {
                ids_to_remove.push(obj.lock().unwrap().borrow_base_object().id);
            }
        }

        // Remove objects from the list
        self.objects
            .retain(|obj| !ids_to_remove.contains(&obj.lock().unwrap().borrow_base_object().id));
    }

    pub fn process_collisions(&self) {
        // Create a vector of indices to sort
        let mut indices: Vec<usize> = (0..self.objects.len()).collect();
        indices.sort_by_key(|&i| self.objects[i].lock().unwrap().borrow_base_object().id);

        for &i in &indices {
            let mut obj = self.objects[i].lock().unwrap();
            for &j in &indices {
                if i == j {
                    continue; // Skip self-collision
                }
                let other = self.objects[j].lock().unwrap();
                obj.try_collide(&*other);
            }
        }
    }
}
