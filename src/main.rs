use macroquad::prelude::*;

mod constants;
mod types;
mod game;
mod render;

use game::Game;
use render::{draw_hud, draw_scene};

#[macroquad::main("African Gun Runners Farming Sim")]
async fn main() {
    // Load all sounds from the `sounds/` folder before the game loop.
    // Any missing MP3 files are silently skipped.
    let sfx = crate::types::SoundEffects::load().await;

    let mut game = Game::new();
    game.sfx = sfx;
    game.load_background().await;


    loop {
        let dt = get_frame_time();
        game.update(dt);
        draw_scene(&game);
        draw_hud(&game);
        next_frame().await;
    }
}
