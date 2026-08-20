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

    // 3. RIVER WATER WITH ANIMATED WAVE OFFSET
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

    // 4. BOATS DOCKED IN WATER
    let wood_boat = Color::from_rgba(125, 80, 45, 255);
    let wood_dark = Color::from_rgba(75, 48, 28, 255);

    // Boat 1
    let b1 = vec3(-30.5, -0.05, -14.0);
    draw_cube(b1, vec3(1.8, 0.5, 4.4), None, wood_boat);
    draw_cube(b1 + vec3(0.0, 0.1, 0.0), vec3(1.4, 0.5, 3.8), None, wood_dark);

    // Boat 2
    let b2 = vec3(-31.8, -0.05, 16.0);
    draw_cube(b2, vec3(2.0, 0.55, 4.8), None, Color::from_rgba(135, 90, 50, 255));
    draw_cube(b2 + vec3(0.0, 0.12, 0.0), vec3(1.6, 0.55, 4.2), None, wood_dark);
    draw_cube(b2 + vec3(0.0, 0.7, 0.0), vec3(2.2, 0.1, 2.4), None, Color::from_rgba(200, 55, 45, 255));

    // 5. WOODEN PLANK BRIDGE
    let plank_color = Color::from_rgba(140, 95, 55, 255);
    let rope_color = Color::from_rgba(190, 160, 100, 255);

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

    for &sx in &[-34.0, -31.0, -28.0] {
        draw_cylinder(vec3(sx, -0.4, -1.9), 0.12, 0.12, 1.2, None, wood_dark);
        draw_cylinder(vec3(sx, -0.4, 1.9), 0.12, 0.12, 1.2, None, wood_dark);
    }

    for &rx in &[-34.5, -31.0, -27.5] {
        draw_cube(vec3(rx, 0.65, -2.0), vec3(0.14, 1.0, 0.14), None, wood_dark);
        draw_cube(vec3(rx, 0.65, 2.0), vec3(0.14, 1.0, 0.14), None, wood_dark);
    }
    draw_line_3d(vec3(-35.0, 1.1, -2.0), vec3(-27.0, 1.1, -2.0), rope_color);
    draw_line_3d(vec3(-35.0, 1.1, 2.0), vec3(-27.0, 1.1, 2.0), rope_color);
}

pub fn draw_market(pos: Vec3, _label: &str, is_near: bool) {
    let wood_dark = Color::from_rgba(75, 48, 28, 255);
    let roof_red = Color::from_rgba(180, 45, 40, 255);
    let counter_wood = Color::from_rgba(135, 90, 50, 255);

    draw_cube(pos, vec3(3.6, 0.2, 3.6), None, wood_dark);
    draw_cube(pos + vec3(0.0, 1.2, 0.0), vec3(3.4, 2.2, 3.4), None, counter_wood);
    draw_cube(pos + vec3(0.0, 2.4, 0.0), vec3(4.0, 0.4, 4.0), None, roof_red);

    let sign_color = if is_near { GOLD } else { YELLOW };
    draw_cube(pos + vec3(0.0, 2.8, 0.0), vec3(2.6, 0.6, 0.2), None, sign_color);
}

pub fn draw_surrounding_houses(game: &Game) {
    for h in &game.houses {
        if !game.camera.is_in_view(h.center, 3.5) {
            continue;
        }

        let (wall_color, roof_color) = match h.style {
            0 => (Color::from_rgba(195, 175, 145, 255), Color::from_rgba(165, 55, 45, 255)),
            1 => (Color::from_rgba(175, 185, 195, 255), Color::from_rgba(50, 70, 95, 255)),
            2 => (Color::from_rgba(205, 190, 150, 255), Color::from_rgba(125, 75, 45, 255)),
            _ => (Color::from_rgba(180, 160, 135, 255), Color::from_rgba(60, 110, 65, 255)),
        };

        draw_cube(h.center + vec3(0.0, 1.3, 0.0), vec3(3.2, 2.6, 3.2), None, wall_color);
        draw_cube(h.center + vec3(0.0, 2.8, 0.0), vec3(3.8, 0.6, 3.8), None, roof_color);

        let door_color = Color::from_rgba(70, 45, 25, 255);
        draw_cube(h.center + vec3(0.0, 0.8, 1.61), vec3(0.9, 1.6, 0.1), None, door_color);

        let window_color = Color::from_rgba(180, 220, 240, 220);
        draw_cube(h.center + vec3(-0.9, 1.5, 1.61), vec3(0.6, 0.6, 0.1), None, window_color);
        draw_cube(h.center + vec3(0.9, 1.5, 1.61), vec3(0.6, 0.6, 0.1), None, window_color);
    }
}
