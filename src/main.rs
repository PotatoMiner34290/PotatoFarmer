use macroquad::prelude::*;

const GRID: usize = 20;
const CELL: f32 = 2.0;
const FIELD_HALF: f32 = GRID as f32 * CELL / 2.0; // 20.0

const GROW_TIME: f32 = 18.0;

// Movement speed in units per second (constant, non-janky speed)
const MOVE_SPEED: f32 = 10.0;
const CAM_SMOOTH: f32 = 8.0;

// Wider camera to view the full field and shacks
const CAM_OFFSET: Vec3 = vec3(22.0, 28.0, 22.0);

const STEP_REPEAT: f32 = 0.12;

// Seed station position moved closer to western grid edge (x = -19.0 is edge cell)
const SEED_STATION_POS: Vec3 = vec3(-FIELD_HALF - 1.2, 0.0, 0.0);
const WATCHTOWER_POS: Vec3 = vec3(FIELD_HALF + 2.5, 0.0, -FIELD_HALF - 2.0);

const POTATO_TO_SEED: u32 = 4;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Grass,
    Plowed,
    Planted { growth: f32 },
}

struct DirtParticle {
    position: Vec3,
    velocity: Vec3,
    life: f32,
    color: Color,
}

struct SparkleParticle {
    position: Vec3,
    velocity: Vec3,
    life: f32,
    max_life: f32,
    color: Color,
}

struct Farmer {
    grid_x: usize,
    grid_z: usize,
    position: Vec3,
    facing: f32,
    plowing: bool,
    step_cooldown: f32,
}

struct CameraState {
    position: Vec3,
    target: Vec3,
}

struct Game {
    field: [[CellState; GRID]; GRID],
    farmer: Farmer,
    camera: CameraState,
    dirt: Vec<DirtParticle>,
    sparkles: Vec<SparkleParticle>,
    seeds: u32,
    potatoes: u32,
    action_cooldown: f32,
    msg_timer: f32,
    status_msg: String,
}

// Deterministic pseudo-random float [0..1] based on grid coordinates and seed index
fn cell_hash(gx: usize, gz: usize, index: u32) -> f32 {
    let mut h = (gx as u32).wrapping_mul(374761393)
        ^ (gz as u32).wrapping_mul(668265263)
        ^ index.wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    (h as f32) / (u32::MAX as f32)
}

impl Game {
    fn new() -> Self {
        let start_x = GRID / 2;
        let start_z = GRID / 2;
        let start_pos = Self::cell_center(start_x, start_z);
        let cam_target = start_pos + vec3(0.0, 0.8, 0.0);

        Self {
            field: [[CellState::Grass; GRID]; GRID],
            farmer: Farmer {
                grid_x: start_x,
                grid_z: start_z,
                position: start_pos,
                facing: 0.0,
                plowing: false,
                step_cooldown: 0.0,
            },
            camera: CameraState {
                position: cam_target + CAM_OFFSET,
                target: cam_target,
            },
            dirt: Vec::new(),
            sparkles: Vec::new(),
            seeds: 24,
            potatoes: 0,
            action_cooldown: 0.0,
            msg_timer: 0.0,
            status_msg: String::new(),
        }
    }

    fn cell_center(grid_x: usize, grid_z: usize) -> Vec3 {
        vec3(
            -FIELD_HALF + grid_x as f32 * CELL + CELL / 2.0,
            0.0,
            -FIELD_HALF + grid_z as f32 * CELL + CELL / 2.0,
        )
    }

    fn near_seed_station(&self) -> bool {
        self.farmer.position.distance(SEED_STATION_POS) < 3.8
    }

    fn set_msg(&mut self, text: &str) {
        self.status_msg = text.to_string();
        self.msg_timer = 3.0;
    }

    fn spawn_dirt(&mut self, pos: Vec3) {
        for _ in 0..8 {
            let shade = (60.0 + rand::gen_range(0.0, 40.0)) as u8;
            self.dirt.push(DirtParticle {
                position: pos + vec3(rand::gen_range(-0.4, 0.4), 0.15, rand::gen_range(-0.4, 0.4)),
                velocity: vec3(
                    rand::gen_range(-2.5, 2.5),
                    rand::gen_range(3.0, 7.0),
                    rand::gen_range(-2.5, 2.5),
                ),
                life: rand::gen_range(0.6, 1.2),
                color: Color::from_rgba(shade, shade / 2 + 10, 20, 255),
            });
        }
    }

    fn spawn_sparkles(&mut self, pos: Vec3) {
        for _ in 0..25 {
            let life = rand::gen_range(0.8, 1.6);
            let colors = [
                Color::from_rgba(255, 215, 0, 255),   // Gold
                Color::from_rgba(255, 165, 0, 255),   // Orange
                Color::from_rgba(100, 220, 100, 255), // Green
                Color::from_rgba(255, 255, 180, 255), // Bright yellow
            ];
            self.sparkles.push(SparkleParticle {
                position: pos
                    + vec3(
                        rand::gen_range(-0.8, 0.8),
                        rand::gen_range(0.2, 1.5),
                        rand::gen_range(-0.8, 0.8),
                    ),
                velocity: vec3(
                    rand::gen_range(-2.0, 2.0),
                    rand::gen_range(2.5, 5.5),
                    rand::gen_range(-2.0, 2.0),
                ),
                life,
                max_life: life,
                color: colors[rand::gen_range(0, colors.len())],
            });
        }
    }

    fn plow_cell(&mut self, gx: usize, gz: usize) {
        if self.field[gx][gz] == CellState::Grass {
            self.field[gx][gz] = CellState::Plowed;
            self.spawn_dirt(Self::cell_center(gx, gz));
        }
    }

    fn plant_cell(&mut self, gx: usize, gz: usize) -> bool {
        if self.seeds == 0 {
            self.set_msg("No seeds left! Convert potatoes at the Seed Station Shack.");
            return false;
        }

        if self.field[gx][gz] == CellState::Plowed {
            self.field[gx][gz] = CellState::Planted { growth: 0.0 };
            self.seeds -= 1;
            return true;
        }

        false
    }

    fn harvest_cell(&mut self, gx: usize, gz: usize) -> bool {
        if let CellState::Planted { growth } = self.field[gx][gz] {
            if growth >= 1.0 {
                self.field[gx][gz] = CellState::Plowed;
                self.potatoes += 1;
                self.spawn_dirt(Self::cell_center(gx, gz));
                return true;
            }
        }
        false
    }

    fn convert_potatoes(&mut self) -> bool {
        if self.potatoes == 0 {
            self.set_msg("No potatoes to convert! Harvest mature potatoes first.");
            return false;
        }

        let converted = self.potatoes * POTATO_TO_SEED;
        let count = self.potatoes;
        self.seeds += converted;
        self.potatoes = 0;
        self.spawn_sparkles(SEED_STATION_POS + vec3(0.0, 1.2, 0.0));
        self.set_msg(&format!("Traded {} Potatoes for {} Seeds!", count, converted));
        true
    }

    // Distance to target cell center
    fn distance_to_cell_center(&self) -> f32 {
        let target = Self::cell_center(self.farmer.grid_x, self.farmer.grid_z);
        self.farmer.position.distance(target)
    }

    fn try_step(&mut self, dx: i32, dz: i32) -> bool {
        let nx = self.farmer.grid_x as i32 + dx;
        let nz = self.farmer.grid_z as i32 + dz;

        if nx < 0 || nz < 0 || nx >= GRID as i32 || nz >= GRID as i32 {
            return false;
        }

        self.farmer.grid_x = nx as usize;
        self.farmer.grid_z = nz as usize;
        self.farmer.facing = (dx as f32).atan2(dz as f32);
        self.farmer.step_cooldown = STEP_REPEAT;

        true
    }

    fn handle_movement_input(&mut self) {
        // Allow responsive queueing when nearing destination cell for buttery smooth continuous movement
        if self.distance_to_cell_center() > 0.25 {
            return;
        }

        let mut dx = 0;
        let mut dz = 0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dz -= 1;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dz += 1;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1;
        }

        if dx != 0 && dz != 0 {
            dx = 0;
        }

        if dx != 0 || dz != 0 {
            self.try_step(dx, dz);
        }
    }

    fn update(&mut self, dt: f32) {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);
        self.farmer.step_cooldown = (self.farmer.step_cooldown - dt).max(0.0);
        self.msg_timer = (self.msg_timer - dt).max(0.0);

        self.handle_movement_input();

        // Buttery smooth constant velocity movement towards target cell
        let target = Self::cell_center(self.farmer.grid_x, self.farmer.grid_z);
        let to_target = target - self.farmer.position;
        let dist = to_target.length();
        if dist > 0.001 {
            let step = (MOVE_SPEED * dt).min(dist);
            self.farmer.position += to_target.normalize() * step;
        } else {
            self.farmer.position = target;
        }
        self.farmer.position.y = 0.0;

        self.farmer.plowing = is_key_down(KeyCode::Space);

        if self.farmer.plowing && self.distance_to_cell_center() < 0.3 {
            self.plow_cell(self.farmer.grid_x, self.farmer.grid_z);
        }

        if is_key_pressed(KeyCode::E) && self.action_cooldown <= 0.0 {
            if self.near_seed_station() {
                self.convert_potatoes();
                self.action_cooldown = 0.4;
            } else {
                let gx = self.farmer.grid_x;
                let gz = self.farmer.grid_z;

                let planted = self.plant_cell(gx, gz);
                let harvested = if !planted {
                    self.harvest_cell(gx, gz)
                } else {
                    false
                };

                if planted || harvested {
                    self.action_cooldown = 0.25;
                }
            }
        }

        // Camera
        let desired_target = self.farmer.position + vec3(0.0, 0.8, 0.0);
        let t = 1.0 - (-CAM_SMOOTH * dt).exp();
        self.camera.target = self.camera.target.lerp(desired_target, t);
        self.camera.position = self.camera.target + CAM_OFFSET;

        // Grow crops
        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                if let CellState::Planted { growth } = cell {
                    *growth = (*growth + dt / GROW_TIME).min(1.0);
                }
            }
        }

        // Dirt physics
        for particle in self.dirt.iter_mut() {
            particle.velocity.y -= 12.0 * dt;
            particle.position += particle.velocity * dt;
            particle.life -= dt;
        }
        self.dirt.retain(|p| p.life > 0.0 && p.position.y > -0.5);

        // Sparkle physics
        for sparkle in self.sparkles.iter_mut() {
            sparkle.velocity.y -= 2.0 * dt;
            sparkle.position += sparkle.velocity * dt;
            sparkle.life -= dt;
        }
        self.sparkles.retain(|s| s.life > 0.0);
    }
}

// Draw high-performance realistic detailed textured soil & field
fn draw_field(game: &Game) {
    for gx in 0..GRID {
        for gz in 0..GRID {
            let center = Game::cell_center(gx, gz);
            let state = game.field[gx][gz];

            match state {
                CellState::Grass => {
                    // Base grass block with organic color variation
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

                    // Small grass tufts on some tiles
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

                    // Dark damp sub-soil base
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

                    // Optimized 3 parallel tilled soil furrows for high FPS and crisp 3D texture
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
                        let ridge_color = Color::from_rgba(r, g, b, 255);

                        // Ridge mound
                        draw_cube(
                            pos,
                            vec3(furrow_w * 0.8, 0.1, CELL * 0.94),
                            None,
                            ridge_color,
                        );
                    }

                    // Optimized soil clods (3 per tile for performance)
                    for c in 0..3 {
                        let h_x = cell_hash(gx, gz, 20 + c * 3);
                        let h_z = cell_hash(gx, gz, 21 + c * 3);
                        let h_s = cell_hash(gx, gz, 22 + c * 3);

                        let clod_pos = center + vec3((h_x - 0.5) * 1.5, 0.07, (h_z - 0.5) * 1.5);
                        let clod_size = vec3(0.12 + h_s * 0.14, 0.07 + h_s * 0.08, 0.12 + h_s * 0.14);

                        let clod_r = (70.0 + h_s * 45.0) as u8;
                        let clod_g = (42.0 + h_s * 30.0) as u8;
                        let clod_b = (20.0 + h_s * 15.0) as u8;
                        draw_cube(
                            clod_pos,
                            clod_size,
                            None,
                            Color::from_rgba(clod_r, clod_g, clod_b, 255),
                        );
                    }
                }
            }

            // Draw crop if planted
            if let CellState::Planted { growth } = state {
                draw_potato_plant(center, growth);
            }
        }
    }
}

fn draw_potato_plant(center: Vec3, growth: f32) {
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

    // Ready to harvest ripe potatoes sticking out of rich soil
    if growth > 0.85 {
        let potato = Color::from_rgba(170, 125, 70, 255);
        draw_sphere(center + vec3(-0.15, 0.14, 0.12), 0.13, None, potato);
        draw_sphere(center + vec3(0.15, 0.12, -0.1), 0.12, None, potato);
        draw_sphere(center + vec3(0.0, 0.15, -0.18), 0.11, None, potato);
    }
}

fn draw_farmer_3d(farmer: &Farmer) {
    let pos = farmer.position;
    let forward = vec3(farmer.facing.sin(), 0.0, farmer.facing.cos());
    let right = vec3(forward.z, 0.0, -forward.x);

    // Boots / Legs
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

    // Camo vest / shirt
    draw_cylinder(
        pos + vec3(0.0, 0.75, 0.0),
        0.28,
        0.24,
        0.8,
        None,
        Color::from_rgba(110, 75, 45, 255),
    );

    // Head (African skin tone)
    draw_sphere(
        pos + vec3(0.0, 1.35, 0.0),
        0.25,
        None,
        Color::from_rgba(85, 50, 30, 255),
    );

    // Nose
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

    // Farming Hoe / Tool
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

// Draw realistic African Gun Runner Shack (Seed Mill) & Fortified Outpost
fn draw_african_gun_runner_shack(pos: Vec3, is_main_station: bool, game: &Game) {
    let wood_dark = Color::from_rgba(85, 55, 35, 255);
    let wood_plank = Color::from_rgba(120, 80, 48, 255);
    let metal_roof = Color::from_rgba(130, 135, 140, 255);
    let metal_rust = Color::from_rgba(165, 75, 45, 255);
    let sandbag_color = Color::from_rgba(185, 165, 120, 255);
    let ammo_green = Color::from_rgba(65, 85, 50, 255);

    // 1. MAIN SHACK BODY
    draw_cube(
        pos + vec3(0.0, 1.3, 0.0),
        vec3(3.2, 2.6, 3.2),
        None,
        wood_dark,
    );

    // Wooden wall cladding stripes
    for i in 0..5 {
        let y_offset = 0.3 + i as f32 * 0.5;
        draw_cube(
            pos + vec3(0.0, y_offset, 1.62),
            vec3(3.1, 0.35, 0.04),
            None,
            wood_plank,
        );
        draw_cube(
            pos + vec3(0.0, y_offset, -1.62),
            vec3(3.1, 0.35, 0.04),
            None,
            wood_plank,
        );
        draw_cube(
            pos + vec3(-1.62, y_offset, 0.0),
            vec3(0.04, 0.35, 3.1),
            None,
            wood_plank,
        );
        draw_cube(
            pos + vec3(1.62, y_offset, 0.0),
            vec3(0.04, 0.35, 3.1),
            None,
            wood_plank,
        );
    }

    // 2. CORRUGATED RUSTY TIN ROOF (Slanted)
    let roof_center = pos + vec3(0.0, 2.85, 0.0);
    draw_cube(roof_center, vec3(3.8, 0.18, 3.8), None, metal_roof);
    // Roof rust patches & corrugated ridges
    for r in 0..4 {
        let rx = -1.4 + r as f32 * 0.93;
        draw_cube(
            roof_center + vec3(rx, 0.12, 0.0),
            vec3(0.4, 0.08, 3.7),
            None,
            metal_rust,
        );
    }

    // 3. FRONT PORCH & SUPPORT LOG PILLARS
    let porch_z = 2.4;
    draw_cylinder(
        pos + vec3(-1.4, 1.1, porch_z),
        0.08,
        0.08,
        2.2,
        None,
        wood_dark,
    );
    draw_cylinder(
        pos + vec3(1.4, 1.1, porch_z),
        0.08,
        0.08,
        2.2,
        None,
        wood_dark,
    );

    // Porch tin canopy
    draw_cube(
        pos + vec3(0.0, 2.3, 2.0),
        vec3(3.6, 0.1, 1.6),
        None,
        metal_rust,
    );

    // Doorway cutout
    draw_cube(
        pos + vec3(0.0, 0.9, 1.61),
        vec3(1.1, 1.8, 0.06),
        None,
        Color::from_rgba(20, 15, 10, 255),
    );

    // 4. DEFENSIVE SANDBAG BARRICADE (Around shack base)
    let sb_h = 0.22;
    let sb_w = 0.45;
    let sb_l = 0.9;

    // Front sandbag wall
    for s in 0..3 {
        let sx = -1.2 + s as f32 * 1.2;
        draw_cube(
            pos + vec3(sx, sb_h * 0.5, 2.3),
            vec3(sb_l, sb_h, sb_w),
            None,
            sandbag_color,
        );
        draw_cube(
            pos + vec3(sx + 0.3, sb_h * 1.5, 2.3),
            vec3(sb_l * 0.85, sb_h, sb_w * 0.85),
            None,
            Color::from_rgba(170, 150, 110, 255),
        );
    }

    // Side sandbags
    draw_cube(
        pos + vec3(-1.8, sb_h * 0.5, 0.8),
        vec3(sb_w, sb_h, sb_l),
        None,
        sandbag_color,
    );
    draw_cube(
        pos + vec3(1.8, sb_h * 0.5, 0.8),
        vec3(sb_w, sb_h, sb_l),
        None,
        sandbag_color,
    );

    // 5. MILITARY AMMO CRATES & CARGO BOXES
    // Green ammo crates stacked on front porch
    draw_cube(
        pos + vec3(1.1, 0.25, 1.9),
        vec3(0.7, 0.5, 0.5),
        None,
        ammo_green,
    );
    draw_cube_wires(
        pos + vec3(1.1, 0.25, 1.9),
        vec3(0.71, 0.51, 0.51),
        BLACK,
    );

    draw_cube(
        pos + vec3(1.15, 0.65, 1.9),
        vec3(0.55, 0.35, 0.45),
        None,
        ammo_green,
    );

    // Wooden supply box on left side
    draw_cube(
        pos + vec3(-1.1, 0.3, 1.8),
        vec3(0.65, 0.6, 0.6),
        None,
        Color::from_rgba(140, 95, 55, 255),
    );

    // 6. RUSTY FUEL BARRELS / OIL DRUMS
    draw_cylinder(
        pos + vec3(-1.9, 0.5, 0.0),
        0.32,
        0.32,
        1.0,
        None,
        Color::from_rgba(180, 50, 40, 255), // Red barrel
    );
    draw_cylinder(
        pos + vec3(-1.9, 0.5, -0.7),
        0.3,
        0.3,
        1.0,
        None,
        Color::from_rgba(45, 80, 135, 255), // Blue barrel
    );

    // 7. AK-47 ASSAULT RIFLE LEANING AGAINST SANDBAGS
    let rifle_pos = pos + vec3(-0.7, 0.5, 2.2);
    // Dark wooden stock
    draw_cube(
        rifle_pos,
        vec3(0.1, 0.35, 0.08),
        None,
        Color::from_rgba(90, 50, 25, 255),
    );
    // Black steel receiver & barrel
    draw_cube(
        rifle_pos + vec3(0.0, 0.35, 0.0),
        vec3(0.06, 0.5, 0.06),
        None,
        BLACK,
    );
    // Curved magazine
    draw_cube(
        rifle_pos + vec3(0.0, 0.25, 0.08),
        vec3(0.05, 0.18, 0.12),
        None,
        DARKGRAY,
    );

    // 8. TALL RADIO MAST ANTENNA
    let antenna_pos = pos + vec3(-1.5, 3.0, -1.5);
    draw_cylinder(antenna_pos, 0.04, 0.06, 3.5, None, DARKGRAY);
    // Red indicator light on top
    draw_sphere(antenna_pos + vec3(0.0, 1.8, 0.0), 0.12, None, RED);

    // 9. RUSTY SIGNBOARD ("SEED & AMMO MILL")
    if is_main_station {
        draw_cube(
            pos + vec3(0.0, 2.35, 2.45),
            vec3(2.2, 0.45, 0.08),
            None,
            Color::from_rgba(160, 110, 50, 255),
        );
        draw_cube_wires(
            pos + vec3(0.0, 2.35, 2.45),
            vec3(2.22, 0.47, 0.1),
            BLACK,
        );

        // Interaction ground ring indicator when player is near station!
        if game.near_seed_station() {
            let pulse = (get_time() * 5.0).sin() as f32 * 0.1;
            draw_cylinder(
                pos + vec3(0.0, 0.02, 2.2),
                1.4 + pulse,
                1.4 + pulse,
                0.04,
                None,
                Color::from_rgba(255, 215, 0, 120),
            );
            draw_cube_wires(
                pos + vec3(0.0, 0.05, 2.2),
                vec3(2.8 + pulse, 0.1, 2.8 + pulse),
                GOLD,
            );
        }
    }
}

fn draw_current_tile_marker(game: &Game) {
    let center = Game::cell_center(game.farmer.grid_x, game.farmer.grid_z);

    draw_cube_wires(
        center + vec3(0.0, 0.3, 0.0),
        vec3(CELL * 0.94, 0.6, CELL * 0.94),
        YELLOW,
    );
}

fn draw_scene(game: &Game) {
    clear_background(Color::from_rgba(135, 195, 235, 255));

    set_camera(&Camera3D {
        position: game.camera.position,
        up: vec3(0.0, 1.0, 0.0),
        target: game.camera.target,
        fovy: 25.0,
        projection: Projection::Orthographics,
        ..Default::default()
    });

    draw_grid(
        GRID as u32,
        CELL,
        Color::from_rgba(40, 40, 40, 60),
        GRAY,
    );

    draw_field(game);

    // Main Gun Runner Seed Station Shack
    draw_african_gun_runner_shack(SEED_STATION_POS, true, game);

    // Secondary Perimeter Outpost Watchtower Shack
    draw_african_gun_runner_shack(WATCHTOWER_POS, false, game);

    // Dirt particles
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

    // Sparkle particles
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

    set_default_camera();
}

fn draw_hud(game: &Game) {
    // Header Panel Background
    draw_rectangle(10.0, 10.0, 420.0, 190.0, Color::from_rgba(20, 25, 30, 200));
    draw_rectangle_lines(10.0, 10.0, 420.0, 190.0, 2.0, GOLD);

    draw_text("AFRICAN GUN RUNNER POTATO FARM", 20.0, 34.0, 20.0, GOLD);
    draw_text("WASD / Arrows - Move Farmer", 20.0, 60.0, 18.0, WHITE);
    draw_text(
        "SPACE - Plow Soil (Hold to till rows)",
        20.0,
        82.0,
        18.0,
        WHITE,
    );
    draw_text(
        "E - Plant / Harvest / Trade at Seed Shack",
        20.0,
        104.0,
        18.0,
        WHITE,
    );

    // Inventory Stats
    let inv_text = format!("Seeds: {}   Potatoes: {}", game.seeds, game.potatoes);
    draw_text(&inv_text, 20.0, 136.0, 24.0, YELLOW);

    // Tile Status
    let gx = game.farmer.grid_x;
    let gz = game.farmer.grid_z;
    let status = match game.field[gx][gz] {
        CellState::Grass => "Tile: Grass (Hold SPACE to plow rich soil)".to_string(),
        CellState::Plowed => "Tile: Plowed Soil (Press E to plant seed)".to_string(),
        CellState::Planted { growth } if growth >= 1.0 => {
            "Tile: Crop Mature! (Press E to harvest potato)".to_string()
        }
        CellState::Planted { growth } => format!("Tile: Growing... {}%", (growth * 100.0) as u32),
    };
    draw_text(&status, 20.0, 164.0, 18.0, LIGHTGRAY);

    // Station Proximity HUD Banner
    if game.near_seed_station() {
        let box_x = screen_width() / 2.0 - 250.0;
        let box_y = screen_height() - 70.0;
        draw_rectangle(box_x, box_y, 500.0, 50.0, Color::from_rgba(30, 40, 20, 230));
        draw_rectangle_lines(box_x, box_y, 500.0, 50.0, 2.0, GOLD);

        draw_text(
            "GUN RUNNER SEED MILL: Press [E] to trade Potatoes -> Seeds (1:4)",
            box_x + 15.0,
            box_y + 32.0,
            20.0,
            GOLD,
        );
    }

    // Temporary Status Feedback Message
    if game.msg_timer > 0.0 {
        let msg_x = screen_width() / 2.0 - 240.0;
        let msg_y = 30.0;
        draw_rectangle(msg_x, msg_y, 480.0, 40.0, Color::from_rgba(180, 50, 40, 220));
        draw_rectangle_lines(msg_x, msg_y, 480.0, 40.0, 2.0, WHITE);
        draw_text(&game.status_msg, msg_x + 15.0, msg_y + 26.0, 18.0, WHITE);
    }
}

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
