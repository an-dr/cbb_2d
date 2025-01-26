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
    objects::{Collectable, Player, Wall},
    sys_collision::SysCollision,
    sys_event::{Event, Subscriber, SysEvent},
    sys_input::SysInput,
    sys_osd::Osd,
    world::World,
};
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Universe {
    sys_event: Arc<Mutex<SysEvent>>,
    sys_input: SysInput,
    sys_collision: SysCollision,
    sys_osd: Osd,
    world: World,
    game_over: bool,
}
const REFERENCE_HEIGHT: f32 = 600.0;
const TIMER: u32 = 30;

impl Universe {
    pub fn new() -> Self {
        let sys_event = Arc::new(Mutex::new(SysEvent::new()));
        let sys_input = SysInput::new(sys_event.clone());
        let sys_collision = SysCollision::new(sys_event.clone());
        let sys_osd = Osd::new();
        let world = World::new();
        Self {
            sys_event,
            sys_input,
            sys_collision,
            sys_osd,
            world,
            game_over: false,
        }
    }

    pub fn add_player(
        &mut self,
        player_id: u32,
        position_xy: (f32, f32),
        size_wh: (f32, f32),
        mass: f32,
        friction: f32,
        color: Color,
    ) {
        let player = Arc::new(Mutex::new(Player::new(
            player_id,
            0,
            position_xy,
            size_wh,
            mass,
            friction,
            color,
        )));
        self.world.add_player_object(player.clone());
        self.sys_event.lock().unwrap().subscribe(player.clone());
        self.sys_collision.add_collidable_object(player.clone());
    }

    pub fn add_wall(&mut self, position_xy: (f32, f32), size_wh: (f32, f32)) {
        let object = Arc::new(Mutex::new(Wall::new(0, position_xy, size_wh)));
        self.push_static_object(object);
    }

    pub fn add_random_collectible(&mut self) {
        let object = Arc::new(Mutex::new(Collectable::random((
            20.0..screen_width() - 20.0,
            20.0..screen_height() - 20.0,
        ))));
        self.world.add_collectable_object(object.clone());
        self.sys_collision.add_collidable_object(object.clone());
    }

    fn cleanup_objects(&mut self) {
        self.world.cleanup_objects();
        self.sys_collision.cleanup_objects();
    }

    fn push_static_object(&mut self, object: Arc<Mutex<Wall>>) {
        self.world.add_object(object.clone());
        self.sys_collision.add_static_object(object.clone());
    }

    pub fn add_random_wall(&mut self) {
        let object = Arc::new(Mutex::new(Wall::random(
            0,
            (0.0..screen_width(), 0.0..screen_height()),
            (30.0..100.0, 30.0..100.0),
        )));
        self.push_static_object(object);
    }

    pub async fn run(&mut self) -> (u32, u32) {
        let (mut p0, mut p1) = (0, 0);
        let mut timer = TIMER;
        let mut last_time = get_time();
        loop {
            let scale = screen_height() / REFERENCE_HEIGHT;

            self.sys_input.read_input();
            self.sys_event.lock().unwrap().process_all();

            clear_background(BLACK);
            self.cleanup_objects();

            self.world.update(get_frame_time());
            // We process collisions after updating the positions
            self.sys_collision.process_collisions();

            let new_p0 = self.world.get_player_score(0);
            let new_p1 = self.world.get_player_score(1);

            for _ in 0..((new_p0 - p0) + (new_p1 - p1)) {
                self.add_random_collectible();
            }
            p0 = new_p0;
            p1 = new_p1;

            // Timer, score, etc.
            let current_time = get_time();
            if current_time - last_time > 1.0 {
                last_time = current_time;
                timer -= 1;
            }
            if timer == 0 {
                self.game_over = true;
            }

            self.sys_osd
                .set_text(format!("TIME: {}, RED:{} BLUE:{}", timer, p0, p1).as_str());

            self.world.draw(scale);
            self.sys_osd.draw(scale);
            next_frame().await;

            if self.game_over {
                break;
            }
        }
        (p0, p1)
    }
}
