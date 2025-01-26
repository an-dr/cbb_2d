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
    base::DynamicAttributes,
    objects::PlayerMode,
    sys_event::{Event, Publisher, SysEvent},
};
use gamepads::Gamepads;
use macroquad::{
    color::{DARKGRAY, GRAY},
    input::{is_key_down, is_key_released, KeyCode},
};
use std::sync::{Arc, Mutex};

pub struct SysInput {
    gamepads: Gamepads,
    event_bus: Arc<Mutex<SysEvent>>,
}

impl SysInput {
    pub fn new(event_bus: Arc<Mutex<SysEvent>>) -> Self {
        Self {
            event_bus,
            gamepads: (Gamepads::new()),
        }
    }

    fn process_gamepads(&mut self) -> (f32, f32, f32, f32) {
        self.gamepads.poll();
        let mut gamepad_count = 0;

        let (mut fx1, mut fy1) = (0.0, 0.0);
        let (mut fx2, mut fy2) = (0.0, 0.0);
        for gamepad in self.gamepads.all() {
            // If A is pressed, change player color
            if gamepad.is_currently_pressed(gamepads::Button::ActionDown) {
                self.publish(Event::SetPlayerMode {
                    id: gamepad_count,
                    mode: PlayerMode::NoClip,
                });
            } else {
                self.publish(Event::SetPlayerMode {
                    id: gamepad_count,
                    mode: PlayerMode::Normal,
                });
            }

            let mut new_fx = 0.0;
            let gamepad_x = gamepad.left_stick_x();
            if gamepad_x.abs() >= 0.1 {
                new_fx = DynamicAttributes::MAX_FORCE * gamepad_x;
            }

            let mut new_fy = 0.0;
            let gamepad_y = gamepad.left_stick_y();
            if gamepad_y.abs() >= 0.1 {
                new_fy = -DynamicAttributes::MAX_FORCE * gamepad_y;
            }

            match gamepad_count {
                0 => {
                    fx1 = new_fx;
                    fy1 = new_fy;
                }
                1 => {
                    fx2 = new_fx;
                    fy2 = new_fy;
                }
                _ => {}
            }

            gamepad_count += 1;
        }
        (fx1, fy1, fx2, fy2)
    }

    fn process_keyboard(&mut self) -> (f32, f32, f32, f32) {
        let (mut fx1, mut fy1) = (0.0, 0.0);
        let (mut fx2, mut fy2) = (0.0, 0.0);

        // Keyboard input
        if is_key_down(KeyCode::Right) {
            fx1 += DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::Left) {
            fx1 -= DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::Up) {
            fy1 -= DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::Down) {
            fy1 += DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::RightControl) {
            self.publish(Event::SetPlayerMode {
                id: 0,
                mode: PlayerMode::NoClip,
            });
        }
        if is_key_released(KeyCode::RightControl) {
            self.publish(Event::SetPlayerMode {
                id: 0,
                mode: PlayerMode::Normal,
            });
        }

        // Keyboard input
        if is_key_down(KeyCode::D) {
            fx2 += DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::A) {
            fx2 -= DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::W) {
            fy2 -= DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::S) {
            fy2 += DynamicAttributes::MAX_FORCE;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.publish(Event::SetPlayerMode {
                id: 1,
                mode: PlayerMode::NoClip,
            });
        }
        if is_key_released(KeyCode::LeftShift) {
            self.publish(Event::SetPlayerMode {
                id: 1,
                mode: PlayerMode::Normal,
            });
        }

        (fx1, fy1, fx2, fy2)
    } // process keyboard

    pub fn read_input(&mut self) {
        let (g_fx1, g_fy1, g_fx2, g_fy2) = self.process_gamepads();
        let (k_fx1, k_fy1, k_fx2, k_fy2) = self.process_keyboard();

        let (fx1, fy1, fx2, fy2) = (g_fx1 + k_fx1, g_fy1 + k_fy1, g_fx2 + k_fx2, g_fy2 + k_fy2);

        self.publish(Event::MovePlayer {
            id: 0,
            fx: fx1,
            fy: fy1,
        });

        self.publish(Event::MovePlayer {
            id: 1,
            fx: fx2,
            fy: fy2,
        });
    }
} // impl

impl Publisher for SysInput {
    fn get_event_bus(&self) -> Arc<Mutex<SysEvent>> {
        self.event_bus.clone()
    }
}
