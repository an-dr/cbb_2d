use macroquad::window::screen_height;

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
use crate::{
    base::{AsAny, Dynamic, GameObject},
    objects::{Collectable, Player},
};
use std::sync::{Arc, Mutex};

pub struct World {
    players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>>,
    objects: Vec<Arc<Mutex<dyn GameObject>>>,
    collectables: Vec<Arc<Mutex<Collectable>>>,
    next_object_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::new())),
            objects: Vec::new(),
            collectables: Vec::new(),
            next_object_id: 0,
        }
    }

    pub fn get_player_score(&self, player_id: u32) -> u32 {
        let mut score = 0;
        for player in self.players.lock().unwrap().iter() {
            if player.lock().unwrap().get_id() == player_id {
                score = player.lock().unwrap().score;
            }
        }
        score
    }

    pub fn add_player_object(&mut self, player: Arc<Mutex<Player>>) {
        self.add_object(player.clone());
        self.players.lock().unwrap().push(player.clone());
    }

    pub fn add_collectable_object(&mut self, collectable: Arc<Mutex<Collectable>>) {
        self.add_object(collectable.clone());
        self.collectables.push(collectable.clone());
    }

    pub fn cleanup_objects(&mut self) {
        //Get IDs of objects to remove
        let mut ids_to_remove = Vec::new();
        for obj in &self.objects {
            if obj
                .lock()
                .unwrap()
                .get_object_attr()
                .is_deletion_requested()
            {
                ids_to_remove.push(obj.lock().unwrap().get_id());
            }
        }

        //Remove objects from the list
        self.objects.retain(|obj| {
            let id = obj.lock().unwrap().get_id();
            !ids_to_remove.contains(&id)
        });
        self.players.lock().unwrap().retain(|player| {
            let id = player.lock().unwrap().get_id();
            !ids_to_remove.contains(&id)
        });
        self.collectables.retain(|collectable| {
            let id = collectable.lock().unwrap().get_id();
            !ids_to_remove.contains(&id)
        });
    }

    fn get_new_object_id(&mut self) -> u32 {
        let id = self.next_object_id;
        self.next_object_id += 1;
        id
    }

    pub fn add_object<T: GameObject + 'static>(&mut self, obj: Arc<Mutex<T>>) {
        let id = self.get_new_object_id();
        obj.lock().unwrap().set_id(id);
        log::debug!("Added {} id: {}", obj.lock().unwrap().get_name(), id);
        self.objects.push(obj);
    }

    pub fn update(&self, delta_time: f32) {
        for player in self.players.lock().unwrap().iter() {
            player.lock().unwrap().update_position(delta_time);
        }
    }

    pub fn draw(&self, scale: f32) {

        for obj in &self.objects {
            let o = obj.lock().unwrap();
            if let Some(_) = obj.as_any().downcast_ref::<Player>() {
                continue;
            }
            o.draw(scale);
        }
        for player in self.players.lock().unwrap().iter() {
            player.lock().unwrap().draw(scale);
        }
    }
}
