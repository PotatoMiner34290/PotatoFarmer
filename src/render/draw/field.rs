use macroquad::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::game::Game;

pub fn draw_field(game: &Game) {
    for gx in 0..GRID {
        for gz in 0..GRID {
            let center = Game::cell_center(gx, gz);
            if !game.camera.is_in_view(center, CELL * 1.5) {
                continue;
            }
            let state = game.field[gx][gz];

            match state {
                CellState::Grass => {
                    let color_variation = cell_hash(gx, gz, 0);
                    let grass_green = Color::from_rgba(
                        (75.0 + color_variation * 20.0) as u8,
                        (145.0 + color_variation * 25.0) as u8,
                        (60.0 + color_variation * 15.0) as u8,
                        255,
                    );
                    draw_cube(
                        center - vec3(0.0, 0.05, 0.0),
                        vec3(CELL * 0.96, 0.1, CELL * 0.96),
                        None,
                        grass_green,
                    );
                }
                CellState::Plowed => {
                    draw_cube(
                        center - vec3(0.0, 0.04, 0.0),
                        vec3(CELL * 0.96, 0.12, CELL * 0.96),
                        None,
                        Color::from_rgba(95, 65, 38, 255),
                    );
                    for r in 0..3 {
                        let offset_z = -0.5 + r as f32 * 0.5;
                        draw_cube(
                            center + vec3(0.0, 0.05, offset_z),
                            vec3(CELL * 0.9, 0.06, 0.28),
                            None,
                            Color::from_rgba(70, 45, 25, 255),
                        );
                    }
                }
                CellState::Planted { growth } => {
                    draw_cube(
                        center - vec3(0.0, 0.04, 0.0),
                        vec3(CELL * 0.96, 0.12, CELL * 0.96),
                        None,
                        Color::from_rgba(95, 65, 38, 255),
                    );

                    let plant_h = 0.2 + growth * 0.65;
                    let stem_color = Color::from_rgba(60, 150, 50, 255);
                    draw_cylinder(
                        center + vec3(0.0, plant_h / 2.0, 0.0),
                        0.08 + growth * 0.05,
                        0.08 + growth * 0.05,
                        plant_h,
                        None,
                        stem_color,
                    );

                    if growth > 0.3 {
                        let leaf_green = Color::from_rgba(40, 180, 60, 255);
                        let leaf_spread = growth * 0.45;
                        draw_sphere(
                            center + vec3(leaf_spread, plant_h * 0.6, 0.0),
                            0.12 + growth * 0.1,
                            None,
                            leaf_green,
                        );
                        draw_sphere(
                            center + vec3(-leaf_spread, plant_h * 0.6, 0.0),
                            0.12 + growth * 0.1,
                            None,
                            leaf_green,
                        );
                        draw_sphere(
                            center + vec3(0.0, plant_h * 0.6, leaf_spread),
                            0.12 + growth * 0.1,
                            None,
                            leaf_green,
                        );
                        draw_sphere(
                            center + vec3(0.0, plant_h * 0.6, -leaf_spread),
                            0.12 + growth * 0.1,
                            None,
                            leaf_green,
                        );
                    }

                    if growth >= 1.0 {
                        let pot_color = Color::from_rgba(180, 130, 70, 255);
                        draw_sphere(center + vec3(0.18, 0.1, 0.15), 0.18, None, pot_color);
                        draw_sphere(center + vec3(-0.15, 0.08, -0.2), 0.16, None, pot_color);
                        draw_sphere(center + vec3(0.0, 0.12, 0.22), 0.17, None, pot_color);
                    }
                }
            }
        }
    }
}

pub fn draw_current_tile_marker(game: &Game) {
    let gx = game.farmer.grid_x;
    let gz = game.farmer.grid_z;

    if gx >= 0 && gx < GRID as i32 && gz >= 0 && gz < GRID as i32 {
        let center = Game::cell_center(gx as usize, gz as usize);
        let pulse = (get_time() * 4.0).sin() as f32 * 0.05;
        draw_cube_wires(
            center + vec3(0.0, 0.1, 0.0),
            vec3(CELL + pulse, 0.3, CELL + pulse),
            GOLD,
        );
    }
}
