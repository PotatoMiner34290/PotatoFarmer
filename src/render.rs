use macroquad::prelude::*;
use crate::types::*;
use crate::game::Game;

pub fn draw_environment() {
    // 1. SUN IN THE SKY WITH MULTI-LAYER GLOW & SUNBEAMS
    let sun_pos = vec3(-30.0, 38.0, -45.0);
    draw_sphere(sun_pos, 4.8, None, Color::from_rgba(255, 240, 150, 255));
    draw_sphere(sun_pos, 7.0, None, Color::from_rgba(255, 200, 60, 90));
    draw_sphere(sun_pos, 10.0, None, Color::from_rgba(255, 170, 30, 40));

    // 2. EXPANDED GROUND TERRAIN
    let ground_y = -0.15;
    let ground_color = Color::from_rgba(65, 120, 55, 255);

    // East Side Ground (Main field & village)
    draw_cube(
        vec3(14.0, ground_y, 0.0),
        vec3(82.0, 0.2, 72.0),
        None,
        ground_color,
    );

    // West Side Ground (Across river)
    draw_cube(
        vec3(-45.0, ground_y, 0.0),
        vec3(20.0, 0.2, 72.0),
        None,
        Color::from_rgba(55, 110, 48, 255),
    );

    // 3. RIVER WATER WITH ANIMATED REFLECTION STRIPES
    let water_color = Color::from_rgba(40, 140, 210, 220);
    let wave_offset = (get_time() * 1.8).sin() as f32 * 0.03;
    draw_cube(
        vec3(-31.0, -0.25 + wave_offset, 0.0),
        vec3(8.0, 0.3, 74.0),
        None,
        water_color,
    );

    // River Mud Banks
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

    // 4. DETAILED BOATS DOCKED IN WATER
    let wood_boat = Color::from_rgba(125, 80, 45, 255);
    let wood_dark = Color::from_rgba(75, 48, 28, 255);

    // Boat 1 (North Dock)
    let b1 = vec3(-30.5, -0.05, -14.0);
    draw_cube(b1, vec3(1.8, 0.5, 4.4), None, wood_boat);
    draw_cube(b1 + vec3(0.0, 0.1, 0.0), vec3(1.4, 0.5, 3.8), None, wood_dark);
    draw_cube(b1 + vec3(0.0, 0.45, 1.2), vec3(1.2, 0.1, 0.8), None, Color::from_rgba(160, 110, 55, 255)); // Bench
    draw_cylinder(b1 + vec3(0.5, 0.4, 0.0), 0.04, 0.04, 2.8, None, DARKGRAY); // Fishing Rod

    // Boat 2 (South Dock with Red Canvas Canopy)
    let b2 = vec3(-31.8, -0.05, 16.0);
    draw_cube(b2, vec3(2.0, 0.55, 4.8), None, Color::from_rgba(135, 90, 50, 255));
    draw_cube(b2 + vec3(0.0, 0.12, 0.0), vec3(1.6, 0.55, 4.2), None, wood_dark);
    draw_cube(b2 + vec3(0.0, 0.7, 0.0), vec3(2.2, 0.1, 2.4), None, Color::from_rgba(200, 55, 45, 255)); // Red Canopy
    draw_cylinder(b2 + vec3(-0.9, 0.4, -0.9), 0.04, 0.04, 0.7, None, DARKGRAY);
    draw_cylinder(b2 + vec3(0.9, 0.4, -0.9), 0.04, 0.04, 0.7, None, DARKGRAY);

    // 5. DETAILED AFRICAN URBAN SHACK WOODEN PLANK BRIDGE
    let plank_color = Color::from_rgba(140, 95, 55, 255);
    let rope_color = Color::from_rgba(190, 160, 100, 255);

    // Individual rustic bridge planks
    for p in 0..16 {
        let px = -35.0 + p as f32 * 0.52;
        let p_alt = if p % 2 == 0 { 0.02 } else { 0.0 };
        draw_cube(
            vec3(px, 0.12 + p_alt, 0.0),
            vec3(0.48, 0.12, 4.3),
            None,
            if p % 3 == 0 { Color::from_rgba(115, 75, 40, 255) } else { plank_color },
        );
    }

    // Underwater Wooden Stilt Supports
    for &sx in &[-34.0, -31.0, -28.0] {
        draw_cylinder(vec3(sx, -0.4, -1.9), 0.12, 0.12, 1.2, None, wood_dark);
        draw_cylinder(vec3(sx, -0.4, 1.9), 0.12, 0.12, 1.2, None, wood_dark);
    }

    // Handrail Posts & Rope Railing
    for &rx in &[-34.5, -31.0, -27.5] {
        draw_cube(vec3(rx, 0.65, -2.0), vec3(0.14, 1.0, 0.14), None, wood_dark);
        draw_cube(vec3(rx, 0.65, 2.0), vec3(0.14, 1.0, 0.14), None, wood_dark);
    }
    draw_line_3d(vec3(-35.0, 1.1, -2.0), vec3(-27.0, 1.1, -2.0), rope_color);
    draw_line_3d(vec3(-35.0, 1.1, 2.0), vec3(-27.0, 1.1, 2.0), rope_color);
}

pub fn draw_air_event_3d(game: &Game) {
    let event = &game.air_event;
    if !event.active && event.bullets.is_empty() {
        return;
    }

    // 1. DETAILED B-2 STEALTH BOMBER
    if event.active {
        let bpos = event.bomber_pos;
        let bomber_dark = Color::from_rgba(35, 38, 42, 255);
        let cockpit_glass = Color::from_rgba(20, 25, 35, 255);

        // Center fuselage
        draw_cube(bpos, vec3(3.4, 0.6, 2.2), None, bomber_dark);
        draw_cube(bpos + vec3(1.8, -0.05, 0.0), vec3(1.4, 0.4, 1.0), None, bomber_dark);

        // Swept-back Wings
        draw_cube(bpos + vec3(-0.5, 0.0, 4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        draw_cube(bpos + vec3(-0.5, 0.0, -4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        draw_cube(bpos + vec3(-2.2, 0.0, 8.5), vec3(2.5, 0.25, 3.5), None, bomber_dark);
        draw_cube(bpos + vec3(-2.2, 0.0, -8.5), vec3(2.5, 0.25, 3.5), None, bomber_dark);

        // Stealth Cockpit
        draw_cube(bpos + vec3(1.2, 0.35, 0.0), vec3(1.0, 0.25, 1.2), None, cockpit_glass);

        // Red Engine Thruster Exhaust Ports
        draw_cube(bpos + vec3(-2.0, 0.1, 1.5), vec3(0.6, 0.3, 1.2), None, RED);
        draw_cube(bpos + vec3(-2.0, 0.1, -1.5), vec3(0.6, 0.3, 1.2), None, RED);

        // 2. DETAILED PURSUIT FIGHTER JETS
        let jet_color = Color::from_rgba(110, 118, 128, 255);
        let jet_canopy = Color::from_rgba(240, 210, 110, 220);

        let render_jet = |jpos: Vec3| {
            draw_cube(jpos, vec3(3.6, 0.55, 0.9), None, jet_color);
            draw_cube(jpos + vec3(2.0, 0.0, 0.0), vec3(1.2, 0.35, 0.4), None, DARKGRAY);
            draw_cube(jpos + vec3(-0.2, 0.0, 0.0), vec3(2.2, 0.15, 3.8), None, jet_color);
            draw_cube(jpos + vec3(-1.4, 0.7, 0.8), vec3(0.8, 0.9, 0.12), None, jet_color);
            draw_cube(jpos + vec3(-1.4, 0.7, -0.8), vec3(0.8, 0.9, 0.12), None, jet_color);
            draw_sphere(jpos + vec3(0.8, 0.35, 0.0), 0.35, None, jet_canopy);
            draw_sphere(jpos + vec3(-1.9, 0.0, 0.0), 0.3, None, ORANGE);
        };

        render_jet(event.jet1_pos);
        render_jet(event.jet2_pos);
    }

    // 3. TRACER BULLETS WITH FIERY TRAILS
    for bullet in &event.bullets {
        draw_sphere(bullet.position, 0.18, None, YELLOW);
        draw_line_3d(bullet.position, bullet.position - bullet.velocity * 0.06, ORANGE);
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

                    // Decorative grass tufts
                    if cell_hash(gx, gz, 1) > 0.7 {
                        let tuft_x = (cell_hash(gx, gz, 2) - 0.5) * 1.2;
                        let tuft_z = (cell_hash(gx, gz, 3) - 0.5) * 1.2;
                        draw_cylinder(
                            center + vec3(tuft_x, 0.08, tuft_z),
                            0.02,
                            0.08,
                            0.18,
                            None,
                            Color::from_rgba(80, 160, 60, 255),
                        );
                    }
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

                    // 3 Plowed soil ridges per cell
                    let num_furrows = 3;
                    let furrow_w = (CELL * 0.92) / num_furrows as f32;
                    for i in 0..num_furrows {
                        let offset_x =
                            -FIELD_HALF + (gx as f32 * CELL) + (i as f32 + 0.5) * furrow_w;
                        let pos = vec3(offset_x, 0.02, center.z);
                        let noise = cell_hash(gx, gz, i as u32 + 10);
                        let r = (90.0 + noise * 30.0 - if is_planted { 15.0 } else { 0.0 }) as u8;
                        let g = (55.0 + noise * 20.0 - if is_planted { 10.0 } else { 0.0 }) as u8;
                        let b = (28.0 + noise * 12.0) as u8;
                        draw_cube(
                            pos,
                            vec3(furrow_w * 0.8, 0.1, CELL * 0.94),
                            None,
                            Color::from_rgba(r, g, b, 255),
                        );
                    }
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

    // Stem
    draw_cylinder(
        center + vec3(0.0, height / 2.0 + 0.08, 0.0),
        0.06,
        0.06,
        height,
        None,
        Color::from_rgba(45, 120, 40, 255),
    );

    // Leaves
    if growth > 0.25 {
        draw_sphere(
            center + vec3(-0.2, 0.45 + growth * 0.4, 0.0),
            0.15 + growth * 0.15,
            None,
            Color::from_rgba(55, 160, 50, 255),
        );

        draw_sphere(
            center + vec3(0.2, 0.5 + growth * 0.4, 0.0),
            0.15 + growth * 0.15,
            None,
            Color::from_rgba(50, 140, 45, 255),
        );
    }

    // Mature Potatoes
    if growth > 0.85 {
        let potato = Color::from_rgba(170, 125, 70, 255);
        draw_sphere(center + vec3(-0.15, 0.14, 0.12), 0.13, None, potato);
        draw_sphere(center + vec3(0.15, 0.12, -0.1), 0.12, None, potato);
        draw_sphere(center + vec3(0.0, 0.15, -0.18), 0.11, None, potato);
    }
}

pub fn draw_farmer_3d(farmer: &Farmer) {
    let pos = farmer.position;
    let forward = vec3(farmer.facing.sin(), 0.0, farmer.facing.cos());
    let right = vec3(forward.z, 0.0, -forward.x);

    // Pants & Legs
    draw_cylinder(
        pos + right * 0.12 + vec3(0.0, 0.25, 0.0),
        0.09,
        0.09,
        0.5,
        None,
        Color::from_rgba(30, 40, 60, 255),
    );
    draw_cylinder(
        pos - right * 0.12 + vec3(0.0, 0.25, 0.0),
        0.09,
        0.09,
        0.5,
        None,
        Color::from_rgba(30, 40, 60, 255),
    );

    // Torso Shirt
    draw_cylinder(
        pos + vec3(0.0, 0.75, 0.0),
        0.28,
        0.24,
        0.8,
        None,
        Color::from_rgba(110, 75, 45, 255),
    );

    // Head & Nose
    draw_sphere(
        pos + vec3(0.0, 1.35, 0.0),
        0.25,
        None,
        Color::from_rgba(85, 50, 30, 255),
    );
    draw_sphere(
        pos + forward * 0.22 + vec3(0.0, 1.35, 0.0),
        0.04,
        None,
        Color::from_rgba(70, 40, 20, 255),
    );

    // Arms
    draw_cylinder(
        pos + right * 0.35 + vec3(0.0, 0.85, 0.0),
        0.07,
        0.07,
        0.55,
        None,
        Color::from_rgba(85, 50, 30, 255),
    );
    draw_cylinder(
        pos - right * 0.35 + vec3(0.0, 0.85, 0.0),
        0.07,
        0.07,
        0.55,
        None,
        Color::from_rgba(85, 50, 30, 255),
    );

    // Straw Hat
    draw_cylinder(
        pos + vec3(0.0, 1.65, 0.0),
        0.15,
        0.48,
        0.35,
        None,
        Color::from_rgba(210, 170, 80, 255),
    );
    draw_cylinder(
        pos + vec3(0.0, 1.52, 0.0),
        0.5,
        0.5,
        0.06,
        None,
        Color::from_rgba(230, 190, 100, 255),
    );

    // Hoe Tool
    draw_line_3d(
        pos + right * 0.35 + vec3(0.0, 0.8, 0.0),
        pos + forward * 0.8 + vec3(0.0, 0.3, 0.0),
        DARKGRAY,
    );

    if farmer.plowing {
        draw_cube(
            pos + forward * 0.9 + vec3(0.0, 0.15, 0.0),
            vec3(0.5, 0.12, 0.35),
            None,
            Color::from_rgba(60, 60, 65, 255),
        );
    }
}

pub fn draw_market(pos: Vec3, _name: &str, is_near: bool) {
    let wood_dark = Color::from_rgba(85, 55, 35, 255);
    let wood_plank = Color::from_rgba(120, 80, 48, 255);
    let metal_roof = Color::from_rgba(130, 135, 140, 255);
    let metal_rust = Color::from_rgba(165, 75, 45, 255);
    let sandbag_color = Color::from_rgba(185, 165, 120, 255);
    let ammo_green = Color::from_rgba(65, 85, 50, 255);

    // 1. MAIN MARKET STRUCTURE
    draw_cube(
        pos + vec3(0.0, 1.3, 0.0),
        vec3(3.4, 2.6, 3.4),
        None,
        wood_dark,
    );

    // Cladding
    for i in 0..5 {
        let y_offset = 0.3 + i as f32 * 0.5;
        draw_cube(
            pos + vec3(0.0, y_offset, 1.72),
            vec3(3.3, 0.35, 0.04),
            None,
            wood_plank,
        );
        draw_cube(
            pos + vec3(0.0, y_offset, -1.72),
            vec3(3.3, 0.35, 0.04),
            None,
            wood_plank,
        );
    }

    // Corrugated Roof
    let roof_center = pos + vec3(0.0, 2.85, 0.0);
    draw_cube(roof_center, vec3(4.0, 0.18, 4.0), None, metal_roof);
    for r in 0..4 {
        let rx = -1.5 + r as f32 * 1.0;
        draw_cube(
            roof_center + vec3(rx, 0.12, 0.0),
            vec3(0.45, 0.08, 3.9),
            None,
            metal_rust,
        );
    }

    // Porch Canopy
    draw_cube(
        pos + vec3(0.0, 2.3, 2.0),
        vec3(3.8, 0.1, 1.6),
        None,
        Color::from_rgba(210, 60, 40, 255),
    );
    for s in 0..3 {
        let sx = -1.2 + s as f32 * 1.2;
        draw_cube(
            pos + vec3(sx, 2.35, 2.0),
            vec3(0.6, 0.12, 1.62),
            None,
            Color::from_rgba(240, 190, 50, 255),
        );
    }

    // Doorway Cutout
    draw_cube(
        pos + vec3(0.0, 0.9, 1.71),
        vec3(1.2, 1.8, 0.06),
        None,
        Color::from_rgba(20, 15, 10, 255),
    );

    // Sandbags & Crates
    for s in 0..3 {
        let sx = -1.3 + s as f32 * 1.3;
        draw_cube(
            pos + vec3(sx, 0.11, 2.3),
            vec3(0.9, 0.22, 0.45),
            None,
            sandbag_color,
        );
    }
    draw_cube(
        pos + vec3(1.2, 0.25, 1.9),
        vec3(0.7, 0.5, 0.5),
        None,
        ammo_green,
    );

    // Leaning AK-47 Rifle
    let rifle_pos = pos + vec3(-0.7, 0.5, 2.2);
    draw_cube(rifle_pos, vec3(0.1, 0.35, 0.08), None, Color::from_rgba(90, 50, 25, 255));
    draw_cube(rifle_pos + vec3(0.0, 0.35, 0.0), vec3(0.06, 0.5, 0.06), None, BLACK);

    // Radio Antenna Mast
    let antenna_pos = pos + vec3(-1.6, 3.0, -1.6);
    draw_cylinder(antenna_pos, 0.04, 0.06, 3.5, None, DARKGRAY);
    draw_sphere(antenna_pos + vec3(0.0, 1.8, 0.0), 0.12, None, RED);

    // Market Signboard
    draw_cube(
        pos + vec3(0.0, 2.35, 2.55),
        vec3(2.4, 0.5, 0.08),
        None,
        Color::from_rgba(40, 70, 30, 255),
    );
    draw_cube_wires(
        pos + vec3(0.0, 2.35, 2.55),
        vec3(2.42, 0.52, 0.1),
        GOLD,
    );

    // 2x2 TRADING GRID AREA DRAWN AROUND MARKET!
    let market_gx = ((pos.x + FIELD_HALF) / CELL).floor() as i32;
    let market_gz = ((pos.z + FIELD_HALF) / CELL).floor() as i32;

    for dx in -1..=1 {
        for dz in -1..=1 {
            let tile_center = Game::grid_to_world(market_gx + dx, market_gz + dz);
            draw_cube_wires(
                tile_center + vec3(0.0, 0.04, 0.0),
                vec3(CELL * 0.98, 0.08, CELL * 0.98),
                if is_near { GOLD } else { Color::from_rgba(70, 180, 240, 180) },
            );
        }
    }

    if is_near {
        let pulse = (get_time() * 5.0).sin() as f32 * 0.1;
        draw_cylinder(
            pos + vec3(0.0, 0.02, 2.2),
            1.4 + pulse,
            1.4 + pulse,
            0.04,
            None,
            Color::from_rgba(255, 215, 0, 140),
        );
    }
}

pub fn draw_surrounding_houses(game: &Game) {
    for h in &game.houses {
        let pos = h.center;
        match h.style {
            // Style 0: Mud-Brick Round Hut with Conical Thatched Roof
            0 => {
                let mud_color = Color::from_rgba(150, 100, 60, 255);
                let thatch_color = Color::from_rgba(215, 170, 80, 255);
                draw_cylinder(pos + vec3(0.0, 1.1, 0.0), 1.4, 1.5, 2.2, None, mud_color);
                draw_cylinder(pos + vec3(0.0, 2.7, 0.0), 0.05, 1.8, 1.3, None, thatch_color);
                draw_cube(pos + vec3(0.0, 0.7, 1.42), vec3(0.7, 1.4, 0.1), None, Color::from_rgba(30, 20, 10, 255));
            }
            // Style 1: Tin Township Shack with Rust Slanted Roof & Porch Overhang
            1 => {
                let tin_color = Color::from_rgba(135, 140, 145, 255);
                let rust_color = Color::from_rgba(175, 80, 45, 255);
                draw_cube(pos + vec3(0.0, 1.1, 0.0), vec3(2.8, 2.2, 2.8), None, tin_color);
                draw_cube(pos + vec3(0.0, 2.3, 0.0), vec3(3.2, 0.15, 3.2), None, rust_color);
                draw_cube(pos + vec3(0.0, 1.9, 1.7), vec3(2.8, 0.08, 1.2), None, rust_color);
                draw_cylinder(pos + vec3(-1.1, 0.9, 2.1), 0.06, 0.06, 1.8, None, DARKGRAY);
                draw_cylinder(pos + vec3(1.1, 0.9, 2.1), 0.06, 0.06, 1.8, None, DARKGRAY);
            }
            // Style 2: White Plaster & Terracotta Tile Villa with Shutters
            2 => {
                let plaster_color = Color::from_rgba(230, 220, 190, 255);
                let tile_color = Color::from_rgba(185, 75, 50, 255);
                draw_cube(pos + vec3(0.0, 1.3, 0.0), vec3(3.0, 2.6, 2.8), None, plaster_color);
                draw_cube(pos + vec3(0.0, 2.75, 0.0), vec3(3.4, 0.3, 3.2), None, tile_color);
                draw_cube(pos + vec3(-0.9, 1.5, 1.42), vec3(0.5, 0.6, 0.05), None, Color::from_rgba(90, 55, 30, 255));
                draw_cube(pos + vec3(0.9, 1.5, 1.42), vec3(0.5, 0.6, 0.05), None, Color::from_rgba(90, 55, 30, 255));
            }
            // Style 3: UNIQUE 4TH HOUSE - 2-Story Fortified Compound Villa!
            3 => {
                let concrete_color = Color::from_rgba(145, 150, 155, 255);
                let sandbag_color = Color::from_rgba(185, 165, 120, 255);

                draw_cube(pos + vec3(0.0, 1.3, 0.0), vec3(3.8, 2.6, 3.8), None, concrete_color);
                draw_cube(pos + vec3(0.0, 3.2, 0.0), vec3(3.2, 1.4, 3.2), None, Color::from_rgba(170, 120, 75, 255));
                draw_cube(pos + vec3(0.0, 4.0, 0.0), vec3(4.2, 0.15, 4.2), None, Color::from_rgba(130, 135, 140, 255));
                draw_cube_wires(pos + vec3(0.0, 3.9, 0.0), vec3(3.4, 0.8, 3.4), BLACK);

                draw_cube(pos + vec3(0.8, 4.2, 0.5), vec3(0.9, 0.06, 0.7), None, Color::from_rgba(30, 70, 140, 255));
                draw_sphere(pos + vec3(-0.8, 4.3, -0.5), 0.25, None, WHITE);

                draw_cube(pos + vec3(-1.2, 0.2, 2.1), vec3(1.2, 0.4, 0.4), None, sandbag_color);
                draw_cube(pos + vec3(1.2, 0.2, 2.1), vec3(1.2, 0.4, 0.4), None, sandbag_color);

                draw_cube(pos + vec3(1.5, 0.3, 1.8), vec3(0.6, 0.6, 0.6), None, Color::from_rgba(65, 85, 50, 255));
                let rifle_pos = pos + vec3(-1.5, 0.5, 2.0);
                draw_cube(rifle_pos, vec3(0.1, 0.35, 0.08), None, Color::from_rgba(90, 50, 25, 255));
                draw_cube(rifle_pos + vec3(0.0, 0.35, 0.0), vec3(0.06, 0.5, 0.06), None, BLACK);
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

    draw_grid(
        GRID as u32,
        CELL,
        Color::from_rgba(40, 40, 40, 60),
        GRAY,
    );

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
