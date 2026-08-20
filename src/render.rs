use macroquad::prelude::*;
use crate::constants::*;
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

pub fn draw_air_event_3d(game: &Game) {
    let event = &game.air_event;
    if !event.active && event.bullets.is_empty() {
        return;
    }

    if event.active {
        let bpos = event.bomber_pos;
        let bomber_dark = Color::from_rgba(35, 38, 42, 255);
        let cockpit_glass = Color::from_rgba(20, 25, 35, 255);

        draw_cube(bpos, vec3(3.4, 0.6, 2.2), None, bomber_dark);
        draw_cube(bpos + vec3(1.8, -0.05, 0.0), vec3(1.4, 0.4, 1.0), None, bomber_dark);

        draw_cube(bpos + vec3(-0.5, 0.0, 4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        draw_cube(bpos + vec3(-0.5, 0.0, -4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        draw_cube(bpos + vec3(-2.2, 0.0, 8.5), vec3(2.5, 0.25, 3.5), None, bomber_dark);
        draw_cube(bpos + vec3(-2.2, 0.0, -8.5), vec3(2.5, 0.25, 3.5), None, bomber_dark);

        draw_cube(bpos + vec3(1.2, 0.35, 0.0), vec3(1.0, 0.25, 1.2), None, cockpit_glass);

        draw_cube(bpos + vec3(-2.0, 0.1, 1.5), vec3(0.6, 0.3, 1.2), None, RED);
        draw_cube(bpos + vec3(-2.0, 0.1, -1.5), vec3(0.6, 0.3, 1.2), None, RED);

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

    for bullet in &event.bullets {
        draw_sphere(bullet.position, 0.18, None, YELLOW);
        draw_line_3d(bullet.position, bullet.position - bullet.velocity * 0.06, ORANGE);
    }
}

// Draw Defensive Guard Turrets
pub fn draw_turrets(game: &Game) {
    let metal_dark = Color::from_rgba(50, 55, 60, 255);
    let gun_green = Color::from_rgba(40, 80, 45, 255);

    for turret in &game.turrets {
        let pos = turret.position;
        if !game.camera.is_in_view(pos, 2.0) {
            continue;
        }

        if !game.turret_meshes.is_empty() {
            // --- OBJ Model render ---
            let scale = 3.5_f32;
            let a = turret.angle;
            let (sin_a, cos_a) = (a.sin(), a.cos());

            for orig_mesh in &game.turret_meshes {
                let transformed_vertices: Vec<Vertex> = orig_mesh
                    .vertices
                    .iter()
                    .map(|v| {
                        let p = v.position * scale;
                        let rot_x = p.x * cos_a + p.z * sin_a;
                        let rot_z = -p.x * sin_a + p.z * cos_a;
                        let world_pos = pos + vec3(rot_x, p.y, rot_z);
                        Vertex {
                            position: world_pos,
                            uv: v.uv,
                            color: v.color,
                            normal: v.normal,
                        }
                    })
                    .collect();

                let transformed_mesh = Mesh {
                    vertices: transformed_vertices,
                    indices: orig_mesh.indices.clone(),
                    texture: orig_mesh.texture.clone(),
                };

                draw_mesh(&transformed_mesh);
            }
        } else {
            // --- Procedural fallback (original cubes/spheres) ---
            // Turret Mount Pedestal
            draw_cylinder(pos + vec3(0.0, 0.4, 0.0), 0.7, 0.6, 0.8, None, metal_dark);
            // Swivel Gun Dome Head — rotated to face aim direction
            let a = turret.angle;
            let barrel_offset = vec3(a.sin() * 0.5, 1.05, a.cos() * 0.5);
            draw_sphere(pos + vec3(0.0, 0.95, 0.0), 0.55, None, gun_green);
            // Twin Heavy Machine Gun Barrels (now aimed at turret.angle)
            let right = vec3(a.cos() * 0.2, 0.0, -a.sin() * 0.2);
            draw_cylinder(pos + barrel_offset + right, 0.08, 0.08, 1.2, None, DARKGRAY);
            draw_cylinder(pos + barrel_offset - right, 0.08, 0.08, 1.2, None, DARKGRAY);
            // Red Targeting Radar Sensor
            draw_sphere(pos + vec3(0.0, 1.35, 0.0), 0.12, None, RED);
        }
    }

    // Render Laser Bullets fired by Turrets
    for bullet in &game.turret_bullets {
        if !game.camera.is_in_view(bullet.position, 1.0) {
            continue;
        }
        draw_sphere(bullet.position, 0.16, None, RED);
        draw_line_3d(bullet.position, bullet.position - bullet.velocity * 0.04, YELLOW);
    }
}


// Draw Thief Children (Detailed small brown children models with running/harvesting animations & Health bar)
pub fn draw_thief_children(game: &Game) {
    let skin_tone = Color::from_rgba(95, 58, 32, 255);
    let shirt_red = Color::from_rgba(205, 55, 45, 255);
    let pants_blue = Color::from_rgba(45, 65, 110, 255);

    for child in &game.children {
        if !child.alive {
            continue;
        }

        let pos = child.position;
        if !game.camera.is_in_view(pos, 2.0) {
            continue;
        }
        let facing = child.facing;
        let is_harvesting = child.harvesting_timer > 0.0 && !child.fleeing;
        let leg_swing = if is_harvesting { 0.0 } else { (child.anim_timer).sin() * 0.25 };

        let forward = vec3(facing.sin(), 0.0, facing.cos());
        let right = vec3(forward.z, 0.0, -forward.x);

        // Legs (cubes instead of cylinders for performance)
        let l_leg_pos = pos + right * 0.09 + forward * leg_swing + vec3(0.0, 0.18, 0.0);
        let r_leg_pos = pos - right * 0.09 - forward * leg_swing + vec3(0.0, 0.18, 0.0);
        draw_cube(l_leg_pos, vec3(0.10, 0.36, 0.10), None, pants_blue);
        draw_cube(r_leg_pos, vec3(0.10, 0.36, 0.10), None, pants_blue);

        // Torso
        let lean_offset = if is_harvesting { forward * 0.15 - vec3(0.0, 0.1, 0.0) } else { forward * 0.05 };
        let torso_pos = pos + vec3(0.0, 0.52, 0.0) + lean_offset;
        draw_cube(torso_pos, vec3(0.32, 0.42, 0.28), None, shirt_red);

        // Arms
        if is_harvesting {
            let reach_pos = torso_pos + forward * 0.2 - vec3(0.0, 0.22, 0.0);
            draw_cube(reach_pos + right * 0.1, vec3(0.08, 0.35, 0.08), None, skin_tone);
            draw_cube(reach_pos - right * 0.1, vec3(0.08, 0.35, 0.08), None, skin_tone);
        } else {
            let arm_swing = (child.anim_timer).cos() * 0.3;
            let l_arm_pos = torso_pos + right * 0.18 - forward * arm_swing;
            let r_arm_pos = torso_pos - right * 0.18 + forward * arm_swing;
            draw_cube(l_arm_pos, vec3(0.08, 0.36, 0.08), None, skin_tone);
            draw_cube(r_arm_pos, vec3(0.08, 0.36, 0.08), None, skin_tone);
        }

        // Head (keep sphere — it's round and visible)
        let head_pos = torso_pos + vec3(0.0, 0.32, 0.0);
        draw_sphere(head_pos, 0.16, None, skin_tone);

        // Floating 3D Health Bar above Thief Head
        let hp_ratio = (child.hp / child.max_hp).clamp(0.0, 1.0);
        let bar_center = head_pos + vec3(0.0, 0.45, 0.0);
        draw_cube(bar_center, vec3(0.8, 0.1, 0.05), None, BLACK);
        if hp_ratio > 0.0 {
            let hp_w = 0.76 * hp_ratio;
            let hp_color = if hp_ratio > 0.5 { GREEN } else { RED };
            draw_cube(bar_center + vec3(-0.38 + hp_w / 2.0, 0.0, 0.01), vec3(hp_w, 0.08, 0.05), None, hp_color);
        }
    }
}

// Draw Israeli Iron Dome Defense Battery
pub fn draw_iron_domes(game: &Game) {
    let dome_base = Color::from_rgba(75, 80, 85, 255);
    let launcher_c = Color::from_rgba(110, 115, 120, 255);
    let missile_c = Color::from_rgba(240, 240, 240, 255);

    for dome in &game.iron_domes {
        let pos = dome.position;
        if !game.camera.is_in_view(pos, 3.0) {
            continue;
        }

        if !game.iron_dome_meshes.is_empty() {
            // --- OBJ Model render ---
            let scale = 0.9_f32;
            let a = dome.angle;
            let (sin_a, cos_a) = (a.sin(), a.cos());

            for orig_mesh in &game.iron_dome_meshes {
                let transformed_vertices: Vec<Vertex> = orig_mesh
                    .vertices
                    .iter()
                    .map(|v| {
                        let p = v.position * scale;
                        let rot_x = p.x * cos_a + p.z * sin_a;
                        let rot_z = -p.x * sin_a + p.z * cos_a;
                        let world_pos = pos + vec3(rot_x, p.y, rot_z);
                        Vertex {
                            position: world_pos,
                            uv: v.uv,
                            color: v.color,
                            normal: v.normal,
                        }
                    })
                    .collect();

                let transformed_mesh = Mesh {
                    vertices: transformed_vertices,
                    indices: orig_mesh.indices.clone(),
                    texture: orig_mesh.texture.clone(),
                };

                draw_mesh(&transformed_mesh);
            }
        } else {
            // Heavy Armored Missile Launcher Base
            draw_cube(pos + vec3(0.0, 0.4, 0.0), vec3(1.8, 0.8, 1.8), None, dome_base);

            // Angled Missile Launch Pod (2x2 Tubes)
            draw_cube(pos + vec3(0.0, 1.1, 0.0), vec3(1.4, 0.9, 1.4), None, launcher_c);
            draw_cylinder(pos + vec3(0.3, 1.5, 0.3), 0.12, 0.12, 0.6, None, DARKGRAY);
            draw_cylinder(pos + vec3(-0.3, 1.5, 0.3), 0.12, 0.12, 0.6, None, DARKGRAY);
            draw_cylinder(pos + vec3(0.3, 1.5, -0.3), 0.12, 0.12, 0.6, None, DARKGRAY);
            draw_cylinder(pos + vec3(-0.3, 1.5, -0.3), 0.12, 0.12, 0.6, None, DARKGRAY);
            // Radar Dish on Side
            draw_sphere(pos + vec3(0.9, 1.0, 0.0), 0.35, None, GOLD);
        }
    }

    // In-flight Iron Dome Missiles (3D missile_iron_dome OBJ model)
    for m in &game.iron_dome_missiles {
        if !game.camera.is_in_view(m.position, 2.0) {
            continue;
        }

        let delta = m.target_pos - m.position;
        let dist = delta.length();
        let dir = if dist > 0.001 { delta / dist } else { vec3(0.0, 1.0, 0.0) };

        if !game.iron_dome_missile_meshes.is_empty() {
            let scale = 3.0_f32;
            let rot = Quat::from_rotation_arc(vec3(0.0, 1.0, 0.0), dir);

            for orig_mesh in &game.iron_dome_missile_meshes {
                let transformed_vertices: Vec<Vertex> = orig_mesh
                    .vertices
                    .iter()
                    .map(|v| {
                        let world_pos = m.position + rot.mul_vec3(v.position * scale);
                        Vertex {
                            position: world_pos,
                            uv: v.uv,
                            color: v.color,
                            normal: v.normal,
                        }
                    })
                    .collect();

                let transformed_mesh = Mesh {
                    vertices: transformed_vertices,
                    indices: orig_mesh.indices.clone(),
                    texture: orig_mesh.texture.clone(),
                };

                draw_mesh(&transformed_mesh);
            }
            // Exhaust fiery trail behind the 3D rocket
            draw_sphere(m.position - dir * 0.4, 0.25, None, ORANGE);
            draw_line_3d(m.position, m.position - dir * 1.6, RED);
        } else {
            draw_cylinder(m.position, 0.1, 0.08, 0.8, None, missile_c);
            draw_sphere(m.position, 0.2, None, ORANGE);
            draw_line_3d(m.position, m.position - vec3(0.0, 1.5, 0.0), RED);
        }
    }
}

// Draw Detailed Cold War Era African Rebel Gunboats
pub fn draw_gunboats(game: &Game) {
    let hull_c = Color::from_rgba(45, 60, 50, 255); // Camo Green Hull
    let deck_c = Color::from_rgba(90, 85, 75, 255);
    let cabin_c = Color::from_rgba(60, 70, 60, 255);
    let turret_c = Color::from_rgba(30, 35, 30, 255);

    for boat in &game.gunboats {
        if !boat.alive {
            continue;
        }
        let pos = boat.position;
        if !game.camera.is_in_view(pos, 5.0) {
            continue;
        }

        // 1. Long Steel Patrol Gunboat Hull
        draw_cube(pos + vec3(0.0, 0.25, 0.0), vec3(2.4, 0.7, 6.5), None, hull_c);
        draw_cube(pos + vec3(0.0, 0.5, 0.0), vec3(2.0, 0.2, 5.8), None, deck_c);

        // 2. Cold War Wheelhouse / Armored Cabin
        draw_cube(pos + vec3(0.0, 1.0, -0.4), vec3(1.6, 0.9, 2.2), None, cabin_c);
        // Radar Mast & Radio Antenna
        draw_cylinder(pos + vec3(0.0, 1.8, -0.4), 0.05, 0.05, 1.0, None, DARKGRAY);
        draw_sphere(pos + vec3(0.0, 2.3, -0.4), 0.2, None, RED);

        // 3. Heavy Mounted Deck Cannon Turret on Bow
        draw_cylinder(pos + vec3(0.0, 0.8, 2.0), 0.45, 0.45, 0.5, None, turret_c);
        draw_cylinder(pos + vec3(0.0, 1.0, 2.4), 0.08, 0.08, 1.2, None, BLACK);

        // 4. Dual Heavy Machine Guns on Stern
        draw_cylinder(pos + vec3(0.5, 0.8, -2.4), 0.06, 0.06, 0.8, None, BLACK);
        draw_cylinder(pos + vec3(-0.5, 0.8, -2.4), 0.06, 0.06, 0.8, None, BLACK);

        // 5. Water Wake & Exhaust Smoke
        draw_sphere(pos + vec3(0.0, 0.0, -3.4), 0.6, None, Color::from_rgba(255, 255, 255, 180));
    }
}

// Draw Armed Disembarked African Rebels & AK-47 Bullets
pub fn draw_rebels(game: &Game) {
    let skin_tone = Color::from_rgba(85, 50, 25, 255);
    let camo_green = Color::from_rgba(45, 75, 40, 255);
    let vest_brown = Color::from_rgba(110, 65, 35, 255);
    let beret_red = Color::from_rgba(190, 40, 30, 255);
    let ak_wood = Color::from_rgba(140, 75, 30, 255);
    let ak_steel = Color::from_rgba(30, 30, 35, 255);

    for rebel in &game.rebels {
        if !rebel.alive {
            continue;
        }
        let pos = rebel.position;
        if !game.camera.is_in_view(pos, 2.0) {
            continue;
        }

        let facing = rebel.facing;
        let leg_swing = (rebel.anim_timer).sin() * 0.2;
        let forward = vec3(facing.sin(), 0.0, facing.cos());
        let right = vec3(forward.z, 0.0, -forward.x);

        // Legs (Camo cargo pants)
        draw_cylinder(pos + right * 0.1 + forward * leg_swing + vec3(0.0, 0.35, 0.0), 0.08, 0.08, 0.7, None, camo_green);
        draw_cylinder(pos - right * 0.1 - forward * leg_swing + vec3(0.0, 0.35, 0.0), 0.08, 0.08, 0.7, None, camo_green);

        // Combat Boots
        draw_cube(pos + right * 0.1 + forward * leg_swing + forward * 0.05 + vec3(0.0, 0.05, 0.0), vec3(0.12, 0.1, 0.22), None, BLACK);
        draw_cube(pos - right * 0.1 - forward * leg_swing + forward * 0.05 + vec3(0.0, 0.05, 0.0), vec3(0.12, 0.1, 0.22), None, BLACK);

        // Torso & Tactical Ammo Vest (African Gun Runner aesthetic)
        let torso_pos = pos + vec3(0.0, 1.05, 0.0);
        draw_cylinder(torso_pos, 0.24, 0.22, 0.75, None, camo_green);
        draw_cube(torso_pos + forward * 0.1, vec3(0.38, 0.6, 0.2), None, vest_brown);
        // Ammo Magazine Pouches
        draw_cube(torso_pos + forward * 0.22 + right * 0.08 - vec3(0.0, 0.1, 0.0), vec3(0.1, 0.2, 0.1), None, DARKGREEN);
        draw_cube(torso_pos + forward * 0.22 - right * 0.08 - vec3(0.0, 0.1, 0.0), vec3(0.1, 0.2, 0.1), None, DARKGREEN);

        // Head & Red Beret
        let head_pos = torso_pos + vec3(0.0, 0.55, 0.0);
        draw_sphere(head_pos, 0.24, None, skin_tone);
        draw_cylinder(head_pos + vec3(0.0, 0.12, 0.0), 0.3, 0.3, 0.08, None, beret_red);

        // AK-47 Assault Rifle with curved banana magazine & wooden stock
        let gun_pos = torso_pos + forward * 0.4 - vec3(0.0, 0.08, 0.0);
        // Wooden Handguard & Receiver Body
        draw_cube(gun_pos, vec3(0.08, 0.1, 0.65), None, ak_wood);
        // Steel Barrel & Sight
        draw_cylinder(gun_pos + forward * 0.35, 0.025, 0.025, 0.5, None, ak_steel);
        // Curved Banana Magazine
        draw_cube(gun_pos - vec3(0.0, 0.14, 0.0) + forward * 0.05, vec3(0.06, 0.22, 0.12), None, ak_steel);

        // Floating 3D Health Bar above African Rebel Head
        let hp_ratio = (rebel.hp / rebel.max_hp).clamp(0.0, 1.0);
        let bar_center = head_pos + vec3(0.0, 0.5, 0.0);
        draw_cube(bar_center, vec3(0.9, 0.12, 0.06), None, BLACK);
        if hp_ratio > 0.0 {
            let hp_w = 0.86 * hp_ratio;
            let hp_color = if hp_ratio > 0.5 { GREEN } else { RED };
            draw_cube(bar_center + vec3(-0.43 + hp_w / 2.0, 0.0, 0.01), vec3(hp_w, 0.09, 0.06), None, hp_color);
        }
    }

    // Render Rebel AK-47 Tracers / Bullets
    for bullet in &game.rebel_bullets {
        if !game.camera.is_in_view(bullet.position, 1.0) {
            continue;
        }
        draw_sphere(bullet.position, 0.15, None, ORANGE);
        draw_line_3d(bullet.position, bullet.position - bullet.velocity * 0.05, YELLOW);
    }
}

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

pub fn draw_farmer_3d(farmer: &Farmer) {
    let pos = farmer.position;
    let facing = farmer.facing;

    let skin_tone = Color::from_rgba(90, 55, 30, 255);
    let shirt_blue = Color::from_rgba(40, 90, 160, 255);
    let pants_brown = Color::from_rgba(80, 50, 25, 255);
    let hat_yellow = Color::from_rgba(220, 180, 50, 255);

    let forward = vec3(facing.sin(), 0.0, facing.cos());
    let right = vec3(forward.z, 0.0, -forward.x);

    let l_leg_pos = pos + right * 0.14 + vec3(0.0, 0.35, 0.0);
    let r_leg_pos = pos - right * 0.14 + vec3(0.0, 0.35, 0.0);
    draw_cylinder(l_leg_pos, 0.09, 0.09, 0.7, None, pants_brown);
    draw_cylinder(r_leg_pos, 0.09, 0.09, 0.7, None, pants_brown);

    let torso_pos = pos + vec3(0.0, 1.05, 0.0);
    draw_cylinder(torso_pos, 0.28, 0.24, 0.75, None, shirt_blue);

    let head_pos = torso_pos + vec3(0.0, 0.55, 0.0);
    draw_sphere(head_pos, 0.26, None, skin_tone);

    let hat_pos = head_pos + vec3(0.0, 0.12, 0.0);
    draw_cylinder(hat_pos, 0.55, 0.55, 0.06, None, hat_yellow);
    draw_sphere(hat_pos + vec3(0.0, 0.1, 0.0), 0.28, None, hat_yellow);

    let l_arm_pos = torso_pos + right * 0.32;
    let r_arm_pos = torso_pos - right * 0.32;
    draw_cylinder(l_arm_pos, 0.07, 0.07, 0.6, None, skin_tone);
    draw_cylinder(r_arm_pos, 0.07, 0.07, 0.6, None, skin_tone);

    if farmer.plowing {
        let hoe_handle_start = torso_pos + forward * 0.3;
        let hoe_handle_end = hoe_handle_start + forward * 0.8 - vec3(0.0, 0.7, 0.0);
        draw_line_3d(hoe_handle_start, hoe_handle_end, Color::from_rgba(120, 80, 45, 255));
        draw_cube(hoe_handle_end, vec3(0.25, 0.05, 0.15), None, GRAY);
    }
}

pub fn draw_market(pos: Vec3, _label: &str, is_near: bool) {
    let building_color = Color::from_rgba(150, 100, 50, 255);
    let roof_color = Color::from_rgba(180, 50, 40, 255);
    let counter_color = Color::from_rgba(100, 65, 35, 255);

    draw_cube(pos + vec3(0.0, 1.0, 0.0), vec3(3.2, 2.0, 3.2), None, building_color);
    draw_cube(pos + vec3(0.0, 2.2, 0.0), vec3(3.6, 0.4, 3.6), None, roof_color);

    let sign_color = if is_near { GOLD } else { WHITE };
    draw_cube(pos + vec3(0.0, 2.6, 0.0), vec3(2.6, 0.5, 0.2), None, sign_color);
    draw_cube(pos + vec3(0.0, 0.5, 1.65), vec3(2.8, 1.0, 0.3), None, counter_color);

    let p1 = pos + vec3(0.4, 1.1, 1.65);
    let p2 = pos + vec3(-0.4, 1.1, 1.65);
    let pot_c = Color::from_rgba(175, 120, 65, 255);
    draw_sphere(p1, 0.22, None, pot_c);
    draw_sphere(p2, 0.22, None, pot_c);
}

pub fn draw_surrounding_houses(game: &Game) {
    let wall_colors = [
        Color::from_rgba(185, 160, 125, 255),
        Color::from_rgba(160, 120, 85, 255),
        Color::from_rgba(200, 180, 150, 255),
        Color::from_rgba(140, 110, 80, 255),
    ];
    let roof_colors = [
        Color::from_rgba(175, 75, 55, 255),
        Color::from_rgba(70, 110, 140, 255),
        Color::from_rgba(150, 115, 60, 255),
        Color::from_rgba(90, 130, 75, 255),
    ];

    for house in &game.houses {
        let center = house.center;
        if !game.camera.is_in_view(center, 4.0) {
            continue;
        }
        let style = house.style;
        let wall_c = wall_colors[style % wall_colors.len()];
        let roof_c = roof_colors[style % roof_colors.len()];

        draw_cube(center + vec3(0.0, 1.0, 0.0), vec3(2.8, 2.0, 2.8), None, wall_c);
        draw_cube(center + vec3(0.0, 2.3, 0.0), vec3(3.2, 0.6, 3.2), None, roof_c);
        draw_cube(center + vec3(0.0, 0.7, 1.42), vec3(0.7, 1.4, 0.08), None, Color::from_rgba(65, 45, 25, 255));
        draw_cube(center + vec3(0.8, 1.2, 1.42), vec3(0.5, 0.5, 0.08), None, Color::from_rgba(180, 220, 240, 255));
        draw_cube(center + vec3(-0.8, 1.2, 1.42), vec3(0.5, 0.5, 0.08), None, Color::from_rgba(180, 220, 240, 255));
    }
}

pub fn draw_dropped_loot(game: &Game) {
    let bounce = (get_time() * 3.5).sin() as f32 * 0.15;
    for loot in &game.dropped_loot {
        if loot.amount == 0 || !game.camera.is_in_view(loot.position, 2.0) {
            continue;
        }
        let pos = loot.position + vec3(0.0, 0.4 + bounce, 0.0);
        match loot.loot_type {
            LootType::BloodDiamonds => {
                draw_sphere(pos, 0.3, None, RED);
                draw_sphere(pos, 0.45, None, Color::from_rgba(255, 50, 80, 100));
            }
            LootType::Cash => {
                draw_cube(pos, vec3(0.5, 0.18, 0.3), None, LIME);
                draw_cube_wires(pos, vec3(0.52, 0.2, 0.32), DARKGREEN);
            }
            LootType::PantherStatue => {
                draw_cylinder(pos - vec3(0.0, 0.15, 0.0), 0.25, 0.25, 0.1, None, BLACK);
                draw_cube(pos, vec3(0.35, 0.4, 0.25), None, Color::from_rgba(40, 20, 60, 255));
                draw_sphere(pos + vec3(0.0, 0.25, 0.0), 0.2, None, PURPLE);
            }
            LootType::Gold => {
                draw_cube(pos, vec3(0.45, 0.2, 0.25), None, GOLD);
                draw_sphere(pos, 0.35, None, Color::from_rgba(255, 215, 0, 120));
            }
            LootType::Bullets => {
                draw_cylinder(pos, 0.12, 0.12, 0.4, None, YELLOW);
                draw_cube(pos, vec3(0.3, 0.25, 0.3), None, Color::from_rgba(180, 140, 40, 255));
            }
            LootType::Minigun => {
                draw_cylinder(pos, 0.18, 0.18, 0.7, None, DARKGRAY);
                draw_cube(pos + vec3(0.0, 0.1, 0.0), vec3(0.3, 0.3, 0.5), None, BLACK);
                draw_sphere(pos, 0.5, None, Color::from_rgba(255, 165, 0, 120));
            }
        }
    }
}

pub fn draw_crashing_bombers(game: &Game) {
    let bomber_dark = Color::from_rgba(40, 42, 48, 255);
    for bomber in &game.crashing_bombers {
        let bpos = bomber.position;
        if !game.camera.is_in_view(bpos, 10.0) {
            continue;
        }
        draw_cube(bpos, vec3(3.4, 0.6, 2.2), None, bomber_dark);
        draw_cube(bpos + vec3(-0.5, 0.0, 4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        draw_cube(bpos + vec3(-0.5, 0.0, -4.5), vec3(3.5, 0.35, 7.5), None, bomber_dark);
        // Fire & smoke trail
        draw_sphere(bpos + vec3(-1.0, 0.2, 0.0), 1.2, None, ORANGE);
        draw_sphere(bpos + vec3(-2.0, 0.5, 0.0), 1.8, None, Color::from_rgba(80, 80, 80, 200));
    }
}

pub fn draw_scene(game: &Game) {
    if game.state != GameState::Playing {
        if let Some(ref bg_tex) = game.menu_background {
            clear_background(DARKGRAY);
            draw_texture_ex(
                bg_tex,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
            // Semi-transparent dark overlay for UI contrast
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(0, 0, 0, 130));
            return;
        } else {
            // No custom image: Render orbit camera 3D background scene!
            clear_background(Color::from_rgba(135, 206, 235, 255));
            let radius = 34.0;
            let cam_x = game.menu_orbit_angle.cos() * radius;
            let cam_z = game.menu_orbit_angle.sin() * radius;
            let cam_pos = vec3(cam_x, 18.0, cam_z);
            let cam_target = vec3(0.0, 0.0, 0.0);

            set_camera(&Camera3D {
                position: cam_pos,
                up: vec3(0.0, -1.0, 0.0),
                target: cam_target,
                fovy: 35.0,
                projection: Projection::Perspective,
                ..Default::default()
            });

            draw_environment();
            draw_grid(GRID as u32, CELL, Color::from_rgba(40, 40, 40, 60), GRAY);
            draw_field(game);
            draw_surrounding_houses(game);
            set_default_camera();

            // Overlay dark vignette tint
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::from_rgba(10, 20, 30, 140));
            return;
        }
    }

    clear_background(Color::from_rgba(135, 206, 235, 255));

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
    if game.camera.is_in_view(WEST_MARKET_POS, 5.0) {
        draw_market(WEST_MARKET_POS, "WEST MARKET", is_near_west);
    }
    if game.camera.is_in_view(EAST_MARKET_POS, 5.0) {
        draw_market(EAST_MARKET_POS, "EAST MARKET", is_near_east);
    }

    draw_surrounding_houses(game);

    // Draw Iron Domes, Gunboats, Rebels, Defense Turrets, Thief Children, Ground Loot and Crashing Bombers
    draw_iron_domes(game);
    draw_gunboats(game);
    draw_rebels(game);
    draw_turrets(game);
    draw_thief_children(game);
    draw_dropped_loot(game);
    draw_crashing_bombers(game);

    for particle in &game.dirt {
        if !game.camera.is_in_view(particle.position, 1.0) {
            continue;
        }
        let alpha = (particle.life * 255.0) as u8;
        // draw_cube is 3-5x faster than draw_sphere in macroquad
        draw_cube(
            particle.position,
            vec3(0.13, 0.13, 0.13),
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
        if !game.camera.is_in_view(sparkle.position, 1.0) {
            continue;
        }
        let progress = sparkle.life / sparkle.max_life;
        let alpha = (progress * 255.0) as u8;
        let size = 0.09 + progress * 0.1;
        // draw_cube is 3-5x faster than draw_sphere
        draw_cube(
            sparkle.position,
            vec3(size, size, size),
            None,
            Color::from_rgba(
                sparkle.color.r as u8,
                sparkle.color.g as u8,
                sparkle.color.b as u8,
                alpha,
            ),
        );
    }

    for smoke in &game.smoke {
        if !game.camera.is_in_view(smoke.position, smoke.size * 2.0) {
            continue;
        }
        let progress = (smoke.life / smoke.max_life).max(0.0).min(1.0);
        let alpha = ((progress * smoke.color.a * 255.0) as u8).min(255);
        let size = smoke.size * (0.65 + (1.0 - progress) * 0.75);
        draw_cube(
            smoke.position,
            vec3(size, size, size),
            None,
            Color::from_rgba(
                (smoke.color.r * 255.0) as u8,
                (smoke.color.g * 255.0) as u8,
                (smoke.color.b * 255.0) as u8,
                alpha,
            ),
        );
    }

    draw_current_tile_marker(game);
    draw_farmer_3d(&game.farmer);
    draw_ai_slaves(game);

    draw_air_event_3d(game);

    set_default_camera();
}

pub fn draw_ai_slaves(game: &Game) {
    let skin_tone = Color::from_rgba(115, 75, 45, 255);
    let shirt_c = Color::from_rgba(200, 140, 50, 255);
    let pants_c = Color::from_rgba(50, 50, 70, 255);
    let hat_c = GOLD;

    for slave in &game.ai_slaves {
        let pos = slave.position;
        if !game.camera.is_in_view(pos, 2.0) {
            continue;
        }
        let facing = slave.facing;
        let leg_swing = (slave.anim_timer).sin() * 0.2;
        let forward = vec3(facing.sin(), 0.0, facing.cos());
        let right = vec3(forward.z, 0.0, -forward.x);

        // Legs — cubes are much faster than cylinders
        draw_cube(pos + right * 0.1 + forward * leg_swing + vec3(0.0, 0.35, 0.0), vec3(0.16, 0.70, 0.16), None, pants_c);
        draw_cube(pos - right * 0.1 - forward * leg_swing + vec3(0.0, 0.35, 0.0), vec3(0.16, 0.70, 0.16), None, pants_c);

        // Torso
        let torso_pos = pos + vec3(0.0, 1.05, 0.0);
        draw_cube(torso_pos, vec3(0.44, 0.75, 0.40), None, shirt_c);

        // Head
        let head_pos = torso_pos + vec3(0.0, 0.55, 0.0);
        draw_sphere(head_pos, 0.22, None, skin_tone);

        // Straw hat brim (flat cube)
        draw_cube(head_pos + vec3(0.0, 0.1, 0.0), vec3(0.9, 0.06, 0.9), None, hat_c);

        // State indicator bar above head
        let bar_center = head_pos + vec3(0.0, 0.4, 0.0);
        draw_cube(bar_center, vec3(0.8, 0.1, 0.05), None, DARKGRAY);
    }
}

pub fn draw_hud(game: &Game) {
    match game.state {
        GameState::MainMenu => {
            draw_main_menu(game);
            return;
        }
        GameState::Controls => {
            draw_main_menu(game);
            draw_controls_overlay();
            return;
        }
        GameState::BgInfo => {
            draw_main_menu(game);
            draw_bg_info_overlay(game);
            return;
        }
        GameState::Playing => {}
    }

    if game.menu_open {
        let pad_x = 40.0;
        let pad_y = 30.0;
        let menu_w = screen_width() - pad_x * 2.0;
        let menu_h = screen_height() - pad_y * 2.0;

        draw_rectangle(pad_x, pad_y, menu_w, menu_h, Color::from_rgba(15, 20, 28, 235));
        draw_rectangle_lines(pad_x, pad_y, menu_w, menu_h, 3.0, GOLD);

        let center_x = screen_width() / 2.0;
        let start_y = pad_y + 50.0;

        draw_text("=== INVENTORY & SPECIAL LOOT MENU ===", center_x - 210.0, start_y, 24.0, GOLD);

        // Core Resources Row
        let mut cur_y = start_y + 40.0;
        draw_text("CORE RESOURCES:", pad_x + 30.0, cur_y, 20.0, SKYBLUE);
        cur_y += 30.0;

        // Draw Seeds Icon & Count
        draw_circle(pad_x + 45.0, cur_y - 6.0, 10.0, GREEN);
        draw_text(&format!("Seeds: {}", game.seeds), pad_x + 65.0, cur_y, 18.0, WHITE);

        // Draw Potatoes Icon & Count
        draw_sphere(vec3(0.0, 0.0, 0.0), 0.0, None, WHITE); // dummy
        draw_circle(pad_x + 220.0, cur_y - 6.0, 10.0, Color::from_rgba(180, 130, 70, 255));
        draw_text(&format!("Potatoes: {}", game.potatoes), pad_x + 240.0, cur_y, 18.0, WHITE);

        // Draw Turrets Inventory
        draw_rectangle(pad_x + 420.0, cur_y - 14.0, 18.0, 18.0, DARKGRAY);
        draw_text(&format!("Turrets: {}", game.turrets_in_inventory), pad_x + 445.0, cur_y, 18.0, WHITE);

        // Draw Iron Domes Inventory
        draw_rectangle(pad_x + 600.0, cur_y - 14.0, 18.0, 18.0, LIGHTGRAY);
        draw_text(&format!("Iron Domes: {}", game.iron_domes_in_inventory), pad_x + 625.0, cur_y, 18.0, WHITE);

        cur_y += 45.0;
        draw_line(pad_x + 20.0, cur_y - 15.0, pad_x + menu_w - 20.0, cur_y - 15.0, 2.0, GRAY);

        // UNLOCKED B-2 BOMBER CURRENCIES & WEAPONS (Only shown after shot down & picked up!)
        draw_text("SPECIAL LOOT (Shot down from B-2 Bomber):", pad_x + 30.0, cur_y, 20.0, GOLD);
        cur_y += 35.0;

        let icon_box_size = 50.0;
        let mut slot_x = pad_x + 30.0;

        // Slot 1: Blood Diamonds
        if game.has_unlocked_blood_diamonds {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, RED);
            // Blood Diamond Icon
            draw_poly(slot_x + 25.0, cur_y + 25.0, 4, 14.0, 45.0, RED);
            draw_text(&format!("x{}", game.blood_diamonds), slot_x + 5.0, cur_y + 65.0, 16.0, RED);
            draw_text("Blood Diamonds", slot_x - 10.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }
        slot_x += 115.0;

        // Slot 2: Cash
        if game.has_unlocked_cash {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, GREEN);
            // Cash Dollar Icon
            draw_rectangle(slot_x + 10.0, cur_y + 14.0, 30.0, 22.0, GREEN);
            draw_text("$", slot_x + 20.0, cur_y + 32.0, 18.0, WHITE);
            draw_text(&format!("${}", game.cash), slot_x + 2.0, cur_y + 65.0, 16.0, GREEN);
            draw_text("Cash Money", slot_x + 2.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }
        slot_x += 115.0;

        // Slot 3: Panther Statue
        if game.has_unlocked_panther_statue {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, PURPLE);
            // Panther Statue Icon
            draw_circle(slot_x + 25.0, cur_y + 25.0, 14.0, PURPLE);
            draw_circle(slot_x + 25.0, cur_y + 25.0, 8.0, BLACK);
            draw_text(&format!("x{}", game.panther_statues), slot_x + 10.0, cur_y + 65.0, 16.0, PURPLE);
            draw_text("Panther Statue", slot_x - 8.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }
        slot_x += 115.0;

        // Slot 4: Gold Bars
        if game.has_unlocked_gold {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, GOLD);
            // Gold Bar Icon
            draw_rectangle(slot_x + 12.0, cur_y + 15.0, 26.0, 20.0, GOLD);
            draw_text(&format!("x{}", game.gold), slot_x + 10.0, cur_y + 65.0, 16.0, GOLD);
            draw_text("Gold Bars", slot_x + 2.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }
        slot_x += 115.0;

        // Slot 5: Bullets
        if game.has_unlocked_bullets {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, YELLOW);
            // Bullet Icon
            draw_rectangle(slot_x + 20.0, cur_y + 12.0, 10.0, 26.0, YELLOW);
            draw_text(&format!("x{}", game.bullets_count), slot_x + 2.0, cur_y + 65.0, 16.0, YELLOW);
            draw_text("Ammo Bullets", slot_x - 5.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }
        slot_x += 115.0;

        // Slot 6: Heavy Minigun
        if game.has_unlocked_minigun {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(30, 40, 50, 255));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 2.0, ORANGE);
            // Minigun Icon
            draw_rectangle(slot_x + 10.0, cur_y + 18.0, 30.0, 14.0, DARKGRAY);
            draw_circle(slot_x + 35.0, cur_y + 25.0, 6.0, ORANGE);
            draw_text("READY", slot_x + 2.0, cur_y + 65.0, 16.0, ORANGE);
            draw_text("Minigun [Auto/F]", slot_x - 10.0, cur_y + 82.0, 14.0, LIGHTGRAY);
        } else {
            draw_rectangle(slot_x, cur_y, icon_box_size, icon_box_size, Color::from_rgba(20, 20, 20, 200));
            draw_rectangle_lines(slot_x, cur_y, icon_box_size, icon_box_size, 1.5, DARKGRAY);
            draw_text("?", slot_x + 18.0, cur_y + 35.0, 24.0, GRAY);
            draw_text("Locked", slot_x + 4.0, cur_y + 65.0, 14.0, DARKGRAY);
        }

        cur_y += 120.0;
        draw_line(pad_x + 20.0, cur_y - 15.0, pad_x + menu_w - 20.0, cur_y - 15.0, 2.0, GRAY);

        if game.game_over {
            draw_text("STATUS: DIED (PERMANENT FAILURE)", pad_x + 30.0, cur_y + 10.0, 22.0, RED);
            draw_text("Press [Y] -> Restart to play again", pad_x + 30.0, cur_y + 40.0, 20.0, YELLOW);
        } else {
            draw_text(&format!("STATUS: ALIVE ({}/100 HP)", game.farmer.hp as u32), pad_x + 30.0, cur_y + 10.0, 20.0, GREEN);
            draw_text("Controls: [TAB] / [ESC] / [V] - Toggle Menu | Minigun Auto-Fires at Threats!", pad_x + 30.0, cur_y + 40.0, 18.0, LIGHTGRAY);
            draw_text("Press [Y] -> Reset & Restart", pad_x + 30.0, cur_y + 65.0, 16.0, YELLOW);
        }
        return;
    }

    if game.game_over {
        // Red Game Over Banner prompting ESC menu
        let go_w = 600.0;
        let go_h = 140.0;
        let go_x = screen_width() / 2.0 - go_w / 2.0;
        let go_y = screen_height() / 2.0 - go_h / 2.0;
        draw_rectangle(go_x, go_y, go_w, go_h, Color::from_rgba(35, 10, 10, 240));
        draw_rectangle_lines(go_x, go_y, go_w, go_h, 3.0, RED);

        draw_text("YOU DIED - PERMANENT FAILURE!", go_x + 60.0, go_y + 45.0, 26.0, RED);
        draw_text("Press [ESC] to open menu and restart new game!", go_x + 35.0, go_y + 90.0, 20.0, GOLD);
        return;
    }

    draw_rectangle(10.0, 10.0, 620.0, 220.0, Color::from_rgba(20, 25, 30, 200));
    draw_rectangle_lines(10.0, 10.0, 620.0, 220.0, 2.0, GOLD);

    draw_text("AFRICAN GUN RUNNER POTATO FARM", 20.0, 34.0, 20.0, GOLD);
    draw_text("WASD / Arrows - Move Grid | SPACE - Plow Rows", 20.0, 60.0, 18.0, WHITE);

    // Player 100 Health Bar
    draw_text("HEALTH:", 20.0, 84.0, 18.0, WHITE);
    let hp_ratio = (game.farmer.hp / game.farmer.max_hp).clamp(0.0, 1.0);
    draw_rectangle(100.0, 72.0, 200.0, 16.0, DARKGRAY);
    let hp_color = if hp_ratio > 0.5 { GREEN } else if hp_ratio > 0.25 { YELLOW } else { RED };
    draw_rectangle(100.0, 72.0, 200.0 * hp_ratio, 16.0, hp_color);
    draw_rectangle_lines(100.0, 72.0, 200.0, 16.0, 1.5, WHITE);
    let hp_str = format!("{}/100", game.farmer.hp as u32);
    draw_text(&hp_str, 310.0, 85.0, 16.0, hp_color);

    draw_text("E - Plant/Harvest | [B] Place Turret | [I] Deploy Iron Dome", 20.0, 114.0, 18.0, WHITE);
    draw_text("F5 / K - Save Game   |   F9 / L - Load Game", 20.0, 136.0, 18.0, SKYBLUE);

    // Render Master Volume Control Bar, Music Mute Button & Hotkey Hint in top right
    let is_music_muted = game.sfx.is_music_muted;
    let vol_pct = game.sfx.volume.clamp(0.0, 1.0);
    
    let btn_size = 22.0;
    let btn_x = screen_width() - btn_size - 15.0;
    let btn_y = 21.0;

    let bar_w = 110.0;
    let bar_h = 14.0;
    let bar_x = btn_x - bar_w - 12.0;
    let bar_y = 25.0;

    let vol_label = format!("Vol: {}%", (vol_pct * 100.0).round() as u32);
    draw_text(&vol_label, bar_x - 70.0, bar_y + 12.0, 16.0, WHITE);

    draw_rectangle(bar_x, bar_y, bar_w, bar_h, DARKGRAY);
    draw_rectangle(bar_x, bar_y, bar_w * vol_pct, bar_h, GREEN);
    draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.5, WHITE);
    draw_circle(bar_x + bar_w * vol_pct, bar_y + bar_h / 2.0, 6.0, GOLD);

    // Mute Music Button Container
    let (mx, my) = mouse_position();
    let is_hover = mx >= btn_x && mx <= btn_x + btn_size && my >= btn_y && my <= btn_y + btn_size;
    let btn_bg = if is_music_muted {
        if is_hover { Color::from_rgba(180, 50, 50, 255) } else { Color::from_rgba(120, 30, 30, 255) }
    } else {
        if is_hover { Color::from_rgba(80, 100, 120, 255) } else { Color::from_rgba(50, 60, 75, 255) }
    };
    draw_rectangle(btn_x, btn_y, btn_size, btn_size, btn_bg);
    draw_rectangle_lines(btn_x, btn_y, btn_size, btn_size, 1.5, if is_music_muted { RED } else { WHITE });

    // Music Note Icon
    let cx = btn_x + btn_size / 2.0;
    let cy = btn_y + btn_size / 2.0;
    let icon_color = if is_music_muted { LIGHTGRAY } else { WHITE };

    draw_circle(cx - 3.5, cy + 3.5, 2.2, icon_color);
    draw_circle(cx + 2.5, cy + 1.5, 2.2, icon_color);
    draw_line(cx - 1.5, cy + 3.5, cx - 1.5, cy - 4.5, 1.8, icon_color);
    draw_line(cx + 4.5, cy + 1.5, cx + 4.5, cy - 6.5, 1.8, icon_color);
    draw_line(cx - 1.5, cy - 4.5, cx + 4.5, cy - 6.5, 2.2, icon_color);

    // Red diagonal cross overlay when music is muted
    if is_music_muted {
        draw_line(btn_x + 3.0, btn_y + 3.0, btn_x + btn_size - 3.0, btn_y + btn_size - 3.0, 2.5, RED);
        draw_line(btn_x + btn_size - 3.0, btn_y + 3.0, btn_x + 3.0, btn_y + btn_size - 3.0, 2.5, RED);
    }

    draw_text("Click/Drag or Ctrl+/- Vol", bar_x - 70.0, bar_y + 32.0, 14.0, LIGHTGRAY);

    let waiting_count = game.ai_slaves.iter().filter(|s| s.state == AiState::WaitingForSeeds).count();
    let inv_text = if waiting_count > 0 {
        format!(
            "Seeds: {}  Potatoes: {}  Slaves: {} ({} waiting for seeds)  Turrets: {}  IronDomes: {}",
            game.seeds, game.potatoes, game.ai_slaves.len(), waiting_count,
            game.turrets_in_inventory, game.iron_domes_in_inventory
        )
    } else {
        format!(
            "Seeds: {}  Potatoes: {}  Slaves: {}  Turrets: {}  IronDomes: {}",
            game.seeds, game.potatoes, game.ai_slaves.len(), game.turrets_in_inventory, game.iron_domes_in_inventory
        )
    };
    draw_text(&inv_text, 20.0, 168.0, 20.0, YELLOW);

    let is_in_field = game.farmer.grid_x >= 0 && game.farmer.grid_x < GRID as i32 &&
                     game.farmer.grid_z >= 0 && game.farmer.grid_z < GRID as i32;

    if is_in_field {
        let gx = game.farmer.grid_x as usize;
        let gz = game.farmer.grid_z as usize;
        match game.field[gx][gz] {
            CellState::Grass => {
                draw_text("Tile: Grass (Hold SPACE to plow rich soil)", 20.0, 188.0, 18.0, LIGHTGRAY);
            }
            CellState::Plowed => {
                draw_text("Tile: Plowed Soil (Press E to plant seed)", 20.0, 188.0, 18.0, LIGHTGRAY);
            }
            CellState::Planted { growth } if growth >= 1.0 => {
                draw_text("Tile: Crop Mature! (Press E to harvest potato)", 20.0, 188.0, 18.0, LIGHTGRAY);
            }
            CellState::Planted { growth } => {
                let status_buf = format!("Tile: Growing... {}%", (growth * 100.0) as u32);
                draw_text(&status_buf, 20.0, 188.0, 18.0, LIGHTGRAY);
            }
        }
    } else {
        draw_text("Exploring Village / River Area", 20.0, 188.0, 18.0, LIGHTGRAY);
    }

    // DEDICATED MARKET SHOP & TYCOON UPGRADE GUI OVERLAY
    if game.near_market() {
        let box_w = 740.0;
        let box_h = if game.market_menu_open { 210.0 } else { 80.0 };
        let box_x = screen_width() / 2.0 - box_w / 2.0;
        let box_y = screen_height() - box_h - 15.0;
        draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(15, 28, 18, 245));
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.5, GOLD);

        if !game.market_menu_open {
            draw_text("=== MARKET SHOP ENTERED ===", box_x + 20.0, box_y + 28.0, 20.0, GOLD);
            draw_text("Press [M] to Open Market & Worker Menu!", box_x + 20.0, box_y + 58.0, 18.0, WHITE);
        } else {
            draw_text("=== DEDICATED MARKET & RYAN'S AUCTION MARTS ===", box_x + 20.0, box_y + 26.0, 20.0, GOLD);
            let slave_mode_label = if game.ai_slave_mode == 0 { "Plant & Harvest" } else { "Plant Only" };
            
            // Only show sell options for currencies already unlocked (hides locked shop items)
            let mut row1_parts: Vec<&str> = Vec::new();
            if game.has_unlocked_panther_statue { row1_parts.push("[1] Sell Panther Statues ($2,500)"); }
            if game.has_unlocked_blood_diamonds { row1_parts.push("[2] Sell Blood Diamonds ($1,500)"); }
            if game.has_unlocked_gold { row1_parts.push("[3] Sell Gold ($200)"); }
            if !row1_parts.is_empty() {
                draw_text(&row1_parts.join(" | "), box_x + 20.0, box_y + 55.0, 16.0, WHITE);
            } else {
                draw_text("No sellable loot unlocked yet - loot B-2 bomber drops to unlock!", box_x + 20.0, box_y + 55.0, 16.0, GRAY);
            }
            draw_text("[4] Trade Potatoes->Seeds | [5] Buy AI Worker Slave (1000 Pot / $500 Cash)", box_x + 20.0, box_y + 85.0, 16.0, SKYBLUE);
            draw_text(&format!("[6] Toggle AI Mode: Current [{}] | [7] Buy +100 Minigun Bullets ($300)", slave_mode_label), box_x + 20.0, box_y + 115.0, 16.0, YELLOW);
            draw_text(&format!("[T] Buy Defense Turret ({} Pot) | [Y] Buy Iron Dome ({} Pot)", TURRET_COST, IRON_DOME_COST), box_x + 20.0, box_y + 145.0, 16.0, GREEN);
            draw_text(&format!("Cash Balance: ${}  |  AI Slaves Hired: {}", game.cash, game.ai_slaves.len()), box_x + 20.0, box_y + 180.0, 18.0, GOLD);
        }
    }

    if game.msg_timer > 0.0 {
        let msg_x = screen_width() / 2.0 - 280.0;
        let msg_y = 30.0;
        draw_rectangle(msg_x, msg_y, 560.0, 40.0, Color::from_rgba(30, 60, 90, 230));
        draw_rectangle_lines(msg_x, msg_y, 560.0, 40.0, 2.0, GOLD);
        draw_text(&game.status_msg, msg_x + 15.0, msg_y + 26.0, 18.0, WHITE);
    }
}

pub fn draw_main_menu(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();

    // 1. Header Title Banner Box
    let title_w = 720.0;
    let title_h = 110.0;
    let title_x = (sw - title_w) / 2.0;
    let title_y = 35.0;

    draw_rectangle(title_x, title_y, title_w, title_h, Color::from_rgba(15, 20, 28, 240));
    draw_rectangle_lines(title_x, title_y, title_w, title_h, 3.0, GOLD);

    draw_text("AFRICAN GUN RUNNERS", title_x + 95.0, title_y + 48.0, 42.0, GOLD);
    draw_text("🌾 POTATO & WEAPON FARMING TYCOON 💣", title_x + 115.0, title_y + 85.0, 22.0, SKYBLUE);

    // 2. Main Menu Options Box
    let menu_w = 560.0;
    let menu_h = 345.0;
    let menu_x = (sw - menu_w) / 2.0;
    let menu_y = title_y + title_h + 25.0;

    draw_rectangle(menu_x, menu_y, menu_w, menu_h, Color::from_rgba(10, 15, 22, 240));
    draw_rectangle_lines(menu_x, menu_y, menu_w, menu_h, 2.5, Color::from_rgba(80, 120, 160, 255));

    let has_save = std::path::Path::new(SAVE_FILE).exists();

    let items = [
        ("▶ NEW GAME [Press N / Enter]", "Start a fresh farming & defense campaign"),
        (if has_save { "💾 CONTINUE / LOAD GAME [Press L]" } else { "💾 CONTINUE / LOAD GAME (No Save Found)" }, if has_save { "Resume your saved progress" } else { "No savefile available yet" }),
        ("⌨ HOW TO PLAY & CONTROLS [Press C]", "View game mechanics, controls & keybindings"),
        ("🚪 QUIT GAME [Press Q]", "Exit to desktop"),
    ];

    let (mx, my) = mouse_position();
    let btn_w = menu_w - 40.0;
    let btn_h = 52.0;
    let start_btn_y = menu_y + 20.0;

    for (i, (label, desc)) in items.iter().enumerate() {
        let cur_y = start_btn_y + i as f32 * 63.0;
        let btn_x = menu_x + 20.0;
        let is_hover = mx >= btn_x && mx <= btn_x + btn_w && my >= cur_y && my <= cur_y + btn_h;
        let is_disabled = i == 1 && !has_save;

        let bg_col = if is_disabled {
            Color::from_rgba(25, 25, 30, 180)
        } else if is_hover {
            Color::from_rgba(45, 80, 120, 255)
        } else {
            Color::from_rgba(20, 30, 42, 220)
        };

        let border_col = if is_disabled {
            DARKGRAY
        } else if is_hover {
            GOLD
        } else {
            Color::from_rgba(70, 95, 120, 255)
        };

        let text_col = if is_disabled {
            GRAY
        } else if is_hover {
            WHITE
        } else {
            LIGHTGRAY
        };

        draw_rectangle(btn_x, cur_y, btn_w, btn_h, bg_col);
        draw_rectangle_lines(btn_x, cur_y, btn_w, btn_h, if is_hover { 2.5 } else { 1.5 }, border_col);

        draw_text(label, btn_x + 15.0, cur_y + 26.0, 20.0, if is_disabled { GRAY } else if is_hover { GOLD } else { WHITE });
        draw_text(desc, btn_x + 15.0, cur_y + 44.0, 14.0, text_col);
    }

    // 3. Custom Background Banner / Syntax Indicator at Bottom
    let banner_w = sw - 60.0;
    let banner_h = 45.0;
    let banner_x = 30.0;
    let banner_y = sh - banner_h - 20.0;

    draw_rectangle(banner_x, banner_y, banner_w, banner_h, Color::from_rgba(12, 18, 26, 240));
    draw_rectangle_lines(banner_x, banner_y, banner_w, banner_h, 2.0, SKYBLUE);

    if let Some(ref path) = game.background_file_name {
        let msg = format!("🟢 CUSTOM BACKGROUND ACTIVE: [{}]  (Drop images into 'assets/menu_bg.png' to change)", path);
        draw_text(&msg, banner_x + 20.0, banner_y + 28.0, 17.0, GREEN);
    } else {
        let msg = "🖼️ CUSTOM BACKGROUND SYNTAX: Drop 'menu_bg.png' or 'background.png' into 'assets/' folder or root! Press [B] for Info";
        draw_text(msg, banner_x + 20.0, banner_y + 28.0, 17.0, YELLOW);
    }
}

pub fn draw_controls_overlay() {
    let sw = screen_width();
    let sh = screen_height();

    let box_w = 740.0;
    let box_h = 490.0;
    let box_x = (sw - box_w) / 2.0;
    let box_y = (sh - box_h) / 2.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(12, 18, 25, 250));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, GOLD);

    draw_text("=== HOW TO PLAY & GAME CONTROLS ===", box_x + 30.0, box_y + 40.0, 24.0, GOLD);

    let controls = [
        ("MOVEMENT", "WASD or Arrow Keys to walk around farm field & river markets"),
        ("PLOWING SOIL", "Hold [SPACE] while moving on grass to plow soil rows"),
        ("PLANT / HARVEST", "Press [E] on plowed soil to plant seeds or harvest mature crops"),
        ("DEDICATED MARKET", "Press [M] near Market structure to buy Worker Slaves & gear"),
        ("DEFENSE TURRETS", "Press [B] to place Automated Defense Turrets (costs potatoes/cash)"),
        ("IRON DOME", "Press [I] to deploy Iron Dome Missile Anti-Air Defense Battery"),
        ("PICKUP STRUCTURE", "Press [P] while near a turret or iron dome to reclaim it"),
        ("WEAPONS / COMBAT", "Minigun auto-fires at incoming threats (Thieves, Gunboats, Jets)"),
        ("SAVE / LOAD", "Press [F5] / [K] to Save Game  |  Press [F9] / [L] to Load Game"),
        ("VOLUME & PAUSE", "Ctrl + '+' / '-' for Volume  |  [ESC] or [TAB] for Pause Menu"),
    ];

    let mut start_y = box_y + 80.0;
    for (category, desc) in controls.iter() {
        draw_text(category, box_x + 30.0, start_y, 16.0, SKYBLUE);
        draw_text(desc, box_x + 220.0, start_y, 16.0, WHITE);
        start_y += 34.0;
    }

    let close_btn_w = 320.0;
    let close_btn_h = 42.0;
    let close_btn_x = box_x + (box_w - close_btn_w) / 2.0;
    let close_btn_y = box_y + box_h - 55.0;

    let (mx, my) = mouse_position();
    let is_hover = mx >= close_btn_x && mx <= close_btn_x + close_btn_w && my >= close_btn_y && my <= close_btn_y + close_btn_h;

    draw_rectangle(close_btn_x, close_btn_y, close_btn_w, close_btn_h, if is_hover { Color::from_rgba(60, 90, 130, 255) } else { Color::from_rgba(30, 45, 65, 230) });
    draw_rectangle_lines(close_btn_x, close_btn_y, close_btn_w, close_btn_h, 2.0, if is_hover { GOLD } else { WHITE });

    draw_text("RETURN TO MENU [ESC / SPACE]", close_btn_x + 20.0, close_btn_y + 27.0, 18.0, if is_hover { GOLD } else { WHITE });
}

pub fn draw_bg_info_overlay(game: &Game) {
    let sw = screen_width();
    let sh = screen_height();

    let box_w = 740.0;
    let box_h = 480.0;
    let box_x = (sw - box_w) / 2.0;
    let box_y = (sh - box_h) / 2.0;

    draw_rectangle(box_x, box_y, box_w, box_h, Color::from_rgba(12, 18, 25, 250));
    draw_rectangle_lines(box_x, box_y, box_w, box_h, 3.0, GOLD);

    draw_text("=== CUSTOM BACKGROUND IMAGE SYNTAX ===", box_x + 30.0, box_y + 40.0, 24.0, GOLD);

    let info_lines = [
        "You can easily set any custom background image for the start menu!",
        "",
        "SYNTAX & STEPS TO ADD YOUR OWN BACKGROUND:",
        "1. Save any PNG or JPG image of your choice.",
        "2. Drop it into the 'assets/' folder (or into the main game root folder).",
        "3. Name the image file as any of the following supported names:",
        "    • menu_bg.png   or   menu_bg.jpg",
        "    • background.png   or   background.jpg",
        "    • menu_background.png   or   menu_background.jpg",
        "4. Launch or restart the game! The engine automatically detects,",
        "   loads, and scales your background image to fit the menu screen.",
        "",
    ];

    let mut start_y = box_y + 80.0;
    for line in info_lines.iter() {
        if line.starts_with("SYNTAX") {
            draw_text(line, box_x + 30.0, start_y, 18.0, SKYBLUE);
        } else if line.trim_start().starts_with("•") {
            draw_text(line, box_x + 30.0, start_y, 17.0, GREEN);
        } else {
            draw_text(line, box_x + 30.0, start_y, 16.0, WHITE);
        }
        start_y += 24.0;
    }

    // Current Active Status
    let status_box_y = start_y + 10.0;
    draw_rectangle(box_x + 30.0, status_box_y, box_w - 60.0, 45.0, Color::from_rgba(20, 30, 45, 230));
    draw_rectangle_lines(box_x + 30.0, status_box_y, box_w - 60.0, 45.0, 1.5, SKYBLUE);

    if let Some(ref path) = game.background_file_name {
        let msg = format!("Current Active Image: [{}]", path);
        draw_text(&msg, box_x + 45.0, status_box_y + 28.0, 18.0, GREEN);
    } else {
        draw_text("Current Active Image: None (Using Procedural 3D Atmosphere)", box_x + 45.0, status_box_y + 28.0, 18.0, YELLOW);
    }

    let close_btn_w = 320.0;
    let close_btn_h = 42.0;
    let close_btn_x = box_x + (box_w - close_btn_w) / 2.0;
    let close_btn_y = box_y + box_h - 55.0;

    let (mx, my) = mouse_position();
    let is_hover = mx >= close_btn_x && mx <= close_btn_x + close_btn_w && my >= close_btn_y && my <= close_btn_y + close_btn_h;

    draw_rectangle(close_btn_x, close_btn_y, close_btn_w, close_btn_h, if is_hover { Color::from_rgba(60, 90, 130, 255) } else { Color::from_rgba(30, 45, 65, 230) });
    draw_rectangle_lines(close_btn_x, close_btn_y, close_btn_w, close_btn_h, 2.0, if is_hover { GOLD } else { WHITE });

    draw_text("RETURN TO MENU [ESC / SPACE]", close_btn_x + 20.0, close_btn_y + 27.0, 18.0, if is_hover { GOLD } else { WHITE });
}



