use macroquad::prelude::*;

mod types;
mod game;
mod render;

use game::Game;
use render::{draw_hud, draw_scene};

#[macroquad::main("African Gun Runner Potato Farmer")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();
        game.update(dt);
        draw_scene(&game);
        draw_hud(&game);
        next_frame().await;
    }
}
