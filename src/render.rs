use macroquad::prelude::*;
use crate::types::*;
use crate::game::Game;

pub fn draw_environment() {
    // 1. SUN IN THE SKY
    let sun_pos = vec3(-30.0, 38.0, -45.0);
    draw_sphere(sun_pos, 4.5, None, Color::from_rgba(255, 235, 120, 255));

    // 2. EXPANDED GROUND TERRAIN
    let ground_y = -0.15;
    let ground_color = Color::from_rgba(65, 120, 55, 255);

    // East Side Ground
    draw_cube(
        vec3(14.0, ground_y, 0.0),
        vec3(82.0, 0.2, 72.0),
        None,
        ground_color,
    );

    // West Side Ground
    draw_cube(
        vec3(-45.0, ground_y, 0.0),
        vec3(20.0, 0.2, 72.0),
        None,
        Color::from_rgba(55, 110, 48, 255),
    );

    // 3. RIVER WATER (Simplified single cube draw for max performance)
    let water_color = Color::from_rgba(40, 140, 210, 210);
    draw_cube(
        vec3(-31.0, -0.25, 0.0),
        vec3(8.0, 0.3, 74.0),
        None,
        water_color,
    );

    // River bed mud banks
    draw_cube(
        vec3(-35.0, -0.1, 0.0),
        vec3(1.5, 0.3, 72.0),
        None,
        Color::from_rgba(90, 65, 40, 255),
    );
    draw_cube(
        vec3(-27.0, -0.1, 0.0),
        vec3(1.5, 0.3, 72.0),
        None,
        Color::from_rgba(90, 65, 40, 255),
    );

    // 4. BOATS DOCKED IN WATER
    let wood_boat = Color::from_rgba(110, 70, 40, 255);
    let wood_dark = Color::from_rgba(70, 45, 25, 255);

    // Boat 1 (North Dock)
    let b1 = vec3(-30.5, -0.05, -14.0);
    draw_cube(b1, vec3(1.8, 0.5, 4.2), None, wood_boat);
    draw_cube(b1 + vec3(0.0, 0.1, 0.0), vec3(1.4, 0.5, 3.8), None, wood_dark);

    // Boat 2 (South Dock)
    let b2 = vec3(-31.8, -0.05, 16.0);
    draw_cube(b2, vec3(2.0, 0.55, 4.6), None, Color::from_rgba(130, 85, 45, 255));
    draw_cube(b2 + vec3(0.0, 0.12, 0.0), vec3(1.6, 0.55, 4.2), None, wood_dark);

    // 5. WOODEN PLANK BRIDGE
    let plank_color = Color::from_rgba(140, 95, 55, 255);
    let rope_color = Color::from_rgba(180, 150, 90, 255);

    // Draw main bridge walkway as unified solid structure
    draw_cube(
        vec3(-31.0, 0.12, 0.0),
        vec3(8.2, 0.12, 4.2),
        None,
        plank_color,
    );

    // Handrail Posts & Rope Railing
    for &rx in &[-34.5, -31.0, -27.5] {
        draw_cube(vec3(rx, 0.65, -2.0), vec3(0.12, 1.0, 0.12), None, wood_dark);
        draw_cube(vec3(rx, 0.65, 2.0), vec3(0.12, 1.0, 0.12), None, wood_dark);
    }
    draw_line_3d(vec3(-35.0, 1.1, -2.0), vec3(-27.0, 1.1, -2.0), rope_color);
    draw_line_3d(vec3(-35.0, 1.1, 2.0), vec3(-27.0, 1.1, 2.0), rope_color);
}

pub fn draw_air_event_3d(game: &Game) {
    let event = &game.air_event;
    if !event.active && event.bullets.is_empty() {
        return;
    }

    // B-2 STEALTH BOMBER
    if event.active {
        let bpos = event.bomber_pos;
        let bomber_dark = Color::from_rgba(35, 38, 42, 255);

        // Core Flying Wing Fuselage
        draw_cube(bpos, vec3(3.2, 0.6, 2.2), None, bomber_dark);
        draw_cube(bpos + vec3(-0.5, 0.0, 0.0), vec3(3.5, 0.35, 15.0), None, bomber_dark);

        // FIGHTER JETS
        let jet_color = Color::from_rgba(110, 118, 128, 255);
        let render_jet = |jpos: Vec3| {
            draw_cube(jpos, vec3(3.6, 0.55, 0.9), None, jet_color);
            draw_cube(jpos + vec3(-0.2, 0.0, 0.0), vec3(2.2, 0.15, 3.8), None, jet_color);
        };

        render_jet(event.jet1_pos);
        render_jet(event.jet2_pos);
    }

    // AIR COMBAT TRACER BULLETS
    for bullet in &event.bullets {
        draw_sphere(bullet.position, 0.15, None, YELLOW);
    }
}

pub fn draw_field(game: &Game) {
    for gx in 0..GRID {
        for gz in 0..GRID {
            let center = Game::cell_center(gx, gz);
            let state = game.field[gx][gz];

            match state {
                CellState::Grass => {
                    let color_variation = cell_hash(gx, gz, 0);
                    let grass_green = Color::from_rgba(
                        (65.0 + color_variation * 18.0) as u8,
                        (125.0 + color_variation * 25.0) as u8,
                        (50.0 + color_variation * 15.0) as u8,
                        255,
                    );
                    draw_cube(
                        center + vec3(0.0, -0.08, 0.0),
                        vec3(CELL * 0.96, 0.16, CELL * 0.96),
                        None,
                        grass_green,
                    );
                }
                CellState::Plowed | CellState::Planted { .. } => {
                    let is_planted = matches!(state, CellState::Planted { .. });
                    let base_color = if is_planted {
                        Color::from_rgba(45, 28, 14, 255)
                    } else {
                        Color::from_rgba(55, 34, 18, 255)
                    };
                    draw_cube(
                        center + vec3(0.0, -0.1, 0.0),
                        vec3(CELL * 0.96, 0.12, CELL * 0.96),
                        None,
                        base_color,
                    );
                }
            }

            if let CellState::Planted { growth } = state {
                draw_potato_plant(center, growth);
            }
        }
    }
}

pub fn draw_potato_plant(center: Vec3, growth: f32) {
    let height = 0.15 + growth * 1.1;

    draw_cylinder(
        center + vec3(0.0, height / 2.0 + 0.08, 0.0),
        0.06,
        0.06,
        height,
        None,
        Color::from_rgba(45, 120, 40, 255),
    );

    if growth > 0.25 {
        draw_sphere(
            center + vec3(0.0, 0.45 + growth * 0.4, 0.0),
            0.2 + growth * 0.15,
            None,
            Color::from_rgba(55, 160, 50, 255),
        );
    }

    if growth > 0.85 {
        let potato = Color::from_rgba(170, 125, 70, 255);
        draw_sphere(center + vec3(0.0, 0.14, 0.0), 0.18, None, potato);
    }
}

pub fn draw_farmer_3d(farmer: &Farmer) {
    let pos = farmer.position;

    // Body & Head
    draw_cylinder(
        pos + vec3(0.0, 0.6, 0.0),
        0.28,
        0.24,
        1.1,
        None,
        Color::from_rgba(110, 75, 45, 255),
    );
    draw_sphere(
        pos + vec3(0.0, 1.35, 0.0),
        0.25,
        None,
        Color::from_rgba(85, 50, 30, 255),
    );

    // Straw Hat
    draw_cylinder(
        pos + vec3(0.0, 1.52, 0.0),
        0.5,
        0.5,
        0.06,
        None,
        Color::from_rgba(230, 190, 100, 255),
    );

    if farmer.plowing {
        draw_cube(
            pos + vec3(0.0, 0.15, 0.6),
            vec3(0.5, 0.12, 0.35),
            None,
            Color::from_rgba(60, 60, 65, 255),
        );
    }
}

pub fn draw_market(pos: Vec3, _name: &str, is_near: bool) {
    let wood_dark = Color::from_rgba(85, 55, 35, 255);
    let metal_roof = Color::from_rgba(130, 135, 140, 255);

    // Main Market building block
    draw_cube(
        pos + vec3(0.0, 1.3, 0.0),
        vec3(3.4, 2.6, 3.4),
        None,
        wood_dark,
    );

    // Roof
    draw_cube(pos + vec3(0.0, 2.85, 0.0), vec3(4.0, 0.18, 4.0), None, metal_roof);

    // Awning
    draw_cube(
        pos + vec3(0.0, 2.3, 2.0),
        vec3(3.8, 0.1, 1.6),
        None,
        Color::from_rgba(210, 60, 40, 255),
    );

    // Radio antenna
    let antenna_pos = pos + vec3(-1.6, 3.0, -1.6);
    draw_cylinder(antenna_pos, 0.04, 0.06, 3.5, None, DARKGRAY);

    // Signboard ("MARKET")
    draw_cube(
        pos + vec3(0.0, 2.35, 2.55),
        vec3(2.4, 0.5, 0.08),
        None,
        Color::from_rgba(40, 70, 30, 255),
    );

    if is_near {
        draw_cube_wires(
            pos + vec3(0.0, 0.05, 2.2),
            vec3(2.8, 0.1, 2.8),
            GOLD,
        );
    }
}

pub fn draw_surrounding_houses(game: &Game) {
    for h in &game.houses {
        let pos = h.center;
        match h.style {
            0 => {
                let mud_color = Color::from_rgba(150, 100, 60, 255);
                let thatch_color = Color::from_rgba(215, 170, 80, 255);
                draw_cylinder(pos + vec3(0.0, 1.1, 0.0), 1.4, 1.5, 2.2, None, mud_color);
                draw_cylinder(pos + vec3(0.0, 2.7, 0.0), 0.05, 1.8, 1.3, None, thatch_color);
            }
            1 => {
                let tin_color = Color::from_rgba(135, 140, 145, 255);
                let rust_color = Color::from_rgba(175, 80, 45, 255);
                draw_cube(pos + vec3(0.0, 1.1, 0.0), vec3(2.8, 2.2, 2.8), None, tin_color);
                draw_cube(pos + vec3(0.0, 2.3, 0.0), vec3(3.2, 0.15, 3.2), None, rust_color);
            }
            2 => {
                let plaster_color = Color::from_rgba(230, 220, 190, 255);
                let tile_color = Color::from_rgba(185, 75, 50, 255);
                draw_cube(pos + vec3(0.0, 1.3, 0.0), vec3(3.0, 2.6, 2.8), None, plaster_color);
                draw_cube(pos + vec3(0.0, 2.75, 0.0), vec3(3.4, 0.3, 3.2), None, tile_color);
            }
            3 => {
                let concrete_color = Color::from_rgba(145, 150, 155, 255);
                draw_cube(pos + vec3(0.0, 1.3, 0.0), vec3(3.8, 2.6, 3.8), None, concrete_color);
                draw_cube(pos + vec3(0.0, 3.2, 0.0), vec3(3.2, 1.4, 3.2), None, Color::from_rgba(170, 120, 75, 255));
                draw_cube(pos + vec3(0.0, 4.0, 0.0), vec3(4.2, 0.15, 4.2), None, Color::from_rgba(130, 135, 140, 255));
            }
            _ => {}
        }
    }
}

pub fn draw_current_tile_marker(game: &Game) {
    let center = Game::grid_to_world(game.farmer.grid_x, game.farmer.grid_z);

    draw_cube_wires(
        center + vec3(0.0, 0.3, 0.0),
        vec3(CELL * 0.94, 0.6, CELL * 0.94),
        YELLOW,
    );
}

pub fn draw_scene(game: &Game) {
    clear_background(Color::from_rgba(135, 195, 235, 255));

    set_camera(&Camera3D {
        position: game.camera.position,
        up: vec3(0.0, 1.0, 0.0),
        target: game.camera.target,
        fovy: 26.0,
        projection: Projection::Orthographics,
        ..Default::default()
    });

    draw_environment();

    draw_field(game);

    let is_near_west = game.farmer.position.distance(WEST_MARKET_POS) < 3.8;
    let is_near_east = game.farmer.position.distance(EAST_MARKET_POS) < 3.8;
    draw_market(WEST_MARKET_POS, "WEST MARKET", is_near_west);
    draw_market(EAST_MARKET_POS, "EAST MARKET", is_near_east);

    draw_surrounding_houses(game);

    for particle in &game.dirt {
        let alpha = (particle.life * 255.0) as u8;
        draw_sphere(
            particle.position,
            0.09,
            None,
            Color::from_rgba(
                particle.color.r as u8,
                particle.color.g as u8,
                particle.color.b as u8,
                alpha,
            ),
        );
    }

    for sparkle in &game.sparkles {
        let progress = sparkle.life / sparkle.max_life;
        let alpha = (progress * 255.0) as u8;
        let size = 0.06 + progress * 0.08;
        draw_sphere(
            sparkle.position,
            size,
            None,
            Color::from_rgba(
                sparkle.color.r as u8,
                sparkle.color.g as u8,
                sparkle.color.b as u8,
                alpha,
            ),
        );
    }

    draw_current_tile_marker(game);
    draw_farmer_3d(&game.farmer);

    draw_air_event_3d(game);

    set_default_camera();
}

pub fn draw_hud(game: &Game) {
    draw_rectangle(10.0, 10.0, 480.0, 215.0, Color::from_rgba(20, 25, 30, 200));
    draw_rectangle_lines(10.0, 10.0, 480.0, 215.0, 2.0, GOLD);

    draw_text("AFRICAN GUN RUNNER POTATO FARM", 20.0, 34.0, 20.0, GOLD);
    draw_text("WASD / Arrows - Move 1 Square at a Time", 20.0, 60.0, 18.0, WHITE);
    draw_text(
        "SPACE - Plow Soil (Hold to till rows)",
        20.0,
        82.0,
        18.0,
        WHITE,
    );
    draw_text(
        "E - Plant / Harvest / Trade at Market",
        20.0,
        104.0,
        18.0,
        WHITE,
    );
    draw_text("F5 / K - Save Game   |   F9 / L - Load Game", 20.0, 126.0, 18.0, SKYBLUE);

    let inv_text = format!("Seeds: {}   Potatoes: {}", game.seeds, game.potatoes);
    draw_text(&inv_text, 20.0, 158.0, 24.0, YELLOW);

    let is_in_field = game.farmer.grid_x >= 0 && game.farmer.grid_x < GRID as i32 &&
                     game.farmer.grid_z >= 0 && game.farmer.grid_z < GRID as i32;

    let status = if is_in_field {
        let gx = game.farmer.grid_x as usize;
        let gz = game.farmer.grid_z as usize;
        match game.field[gx][gz] {
            CellState::Grass => "Tile: Grass (Hold SPACE to plow rich soil)".to_string(),
            CellState::Plowed => "Tile: Plowed Soil (Press E to plant seed)".to_string(),
            CellState::Planted { growth } if growth >= 1.0 => {
                "Tile: Crop Mature! (Press E to harvest potato)".to_string()
            }
            CellState::Planted { growth } => format!("Tile: Growing... {}%", (growth * 100.0) as u32),
        }
    } else {
        "Exploring Village / River Area".to_string()
    };
    draw_text(&status, 20.0, 188.0, 18.0, LIGHTGRAY);

    if game.near_market() {
        let box_x = screen_width() / 2.0 - 250.0;
        let box_y = screen_height() - 70.0;
        draw_rectangle(box_x, box_y, 500.0, 50.0, Color::from_rgba(30, 40, 20, 230));
        draw_rectangle_lines(box_x, box_y, 500.0, 50.0, 2.0, GOLD);

        draw_text(
            "MARKET TRADER: Press [E] to trade Potatoes -> Seeds (1:4)",
            box_x + 20.0,
            box_y + 32.0,
            20.0,
            GOLD,
        );
    }

    if game.msg_timer > 0.0 {
        let msg_x = screen_width() / 2.0 - 260.0;
        let msg_y = 30.0;
        draw_rectangle(msg_x, msg_y, 520.0, 40.0, Color::from_rgba(30, 60, 90, 230));
        draw_rectangle_lines(msg_x, msg_y, 520.0, 40.0, 2.0, GOLD);
        draw_text(&game.status_msg, msg_x + 15.0, msg_y + 26.0, 18.0, WHITE);
    }
}
