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

use cbb_2d;
use log::*;
use macroquad::prelude::*;
use simplelog::*;
use std::panic;

fn window_conf() -> Conf {
    Conf {
        window_title: "rgame".to_string(),
        window_resizable: false,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if cfg!(debug_assertions) {
        init_log();
    }
    loop {
        let (p0, p1) = new_game().await;
        game_over(p0, p1).await;
    }
}

fn init_log() {
    SimpleLogger::init(LevelFilter::Debug, Config::default()).unwrap();
    log::debug!("Debug build: logging enabled");
    panic::set_hook(Box::new(|e| {
        log::error!("{e}");
    }));
}

async fn game_over(p0: u32, p1: u32) {
    let mut gamepads = gamepads::Gamepads::new();

    'game_over: loop {
        clear_background(BLACK);
        draw_text("Game Over", 100.0, 100.0, 30.0, WHITE);
        draw_text(&format!("RED: {}", p0), 100.0, 120.0, 30.0, WHITE);
        draw_text(&format!("BLUE: {}", p1), 100.0, 140.0, 30.0, WHITE);
        draw_text("Press Enter/A to a New Game", 100.0, 200.0, 30.0, WHITE);

        gamepads.poll();
        for gamepad in gamepads.all() {
            if gamepad.is_currently_pressed(gamepads::Button::ActionDown) {
                break 'game_over;
            }
        }
        if is_key_down(KeyCode::Enter) {
            break 'game_over;
        }
        next_frame().await;
    }
}

async fn new_game() -> (u32, u32) {
    let mut universe = cbb_2d::universe::Universe::new();
    universe.add_player(0, (100.0, 100.0), (30.0, 30.0), 0.5, 0.2, RED);
    universe.add_player(1, (200.0, 200.0), (20.0, 20.0), 2.0, 0.05, BLUE);

    // Add walls around the screen, leaving only on_screen_width
    let thikness = 1000.0;
    let on_screen_width = 10.0;
    universe.add_wall(
        (0.0 - on_screen_width, -thikness + on_screen_width),
        (screen_width() + 2.0 * on_screen_width, thikness),
    );
    universe.add_wall(
        (screen_width() - on_screen_width, 0.0 - on_screen_width),
        (thikness, screen_height() + 2.0 * on_screen_width),
    );
    universe.add_wall(
        (0.0 - on_screen_width, screen_height() - on_screen_width),
        (screen_width() + 2.0 * on_screen_width, thikness),
    );
    universe.add_wall(
        (-thikness + on_screen_width, 0.0 - on_screen_width),
        (thikness, screen_height() + 2.0 * on_screen_width),
    );

    for _ in 0..40 {
        universe.add_random_wall();
    }

    for _ in 0..10 {
        universe.add_random_collectible();
    }

    universe.run().await
}
