use macroquad::prelude::*;
use std::fs::File;
use std::io::{Read, Write};

use crate::types::*;

pub struct Game {
    pub field: [[CellState; GRID]; GRID],
    pub farmer: Farmer,
    pub camera: CameraState,
    pub dirt: Vec<DirtParticle>,
    pub sparkles: Vec<SparkleParticle>,
    pub seeds: u32,
    pub potatoes: u32,
    pub action_cooldown: f32,
    pub msg_timer: f32,
    pub status_msg: String,
    pub air_event: AirEvent,
}

impl Game {
    pub fn new() -> Self {
        let start_x = GRID / 2;
        let start_z = GRID / 2;
        let start_pos = Self::cell_center(start_x, start_z);
        let cam_target = start_pos + vec3(0.0, 0.8, 0.0);

        let mut game = Self {
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
            air_event: AirEvent {
                active: false,
                timer: 50.0, // Trigger first flyby shortly after starting (10s in)
                fly_time: 0.0,
                bomber_pos: Vec3::ZERO,
                jet1_pos: Vec3::ZERO,
                jet2_pos: Vec3::ZERO,
                bullets: Vec::new(),
            },
        };

        if std::path::Path::new(SAVE_FILE).exists() {
            game.load_game();
            game.set_msg("Welcome back! Auto-loaded saved game.");
        }

        game
    }

    pub fn cell_center(grid_x: usize, grid_z: usize) -> Vec3 {
        vec3(
            -FIELD_HALF + grid_x as f32 * CELL + CELL / 2.0,
            0.0,
            -FIELD_HALF + grid_z as f32 * CELL + CELL / 2.0,
        )
    }

    pub fn near_market(&self) -> bool {
        self.farmer.position.distance(WEST_MARKET_POS) < 3.8
            || self.farmer.position.distance(EAST_MARKET_POS) < 3.8
    }

    pub fn active_market_pos(&self) -> Vec3 {
        if self.farmer.position.distance(WEST_MARKET_POS)
            < self.farmer.position.distance(EAST_MARKET_POS)
        {
            WEST_MARKET_POS
        } else {
            EAST_MARKET_POS
        }
    }

    pub fn set_msg(&mut self, text: &str) {
        self.status_msg = text.to_string();
        self.msg_timer = 3.5;
    }

    pub fn save_game(&mut self) {
        let field_save: Vec<Vec<CellStateSave>> = self
            .field
            .iter()
            .map(|row| row.iter().map(|c| CellStateSave::from(*c)).collect())
            .collect();

        let save_data = SaveData {
            seeds: self.seeds,
            potatoes: self.potatoes,
            farmer_grid_x: self.farmer.grid_x,
            farmer_grid_z: self.farmer.grid_z,
            field: field_save,
        };

        if let Ok(json) = serde_json::to_string_pretty(&save_data) {
            if let Ok(mut file) = File::create(SAVE_FILE) {
                if file.write_all(json.as_bytes()).is_ok() {
                    self.set_msg("Game Saved to savegame.json!");
                    return;
                }
            }
        }
        self.set_msg("Failed to save game!");
    }

    pub fn load_game(&mut self) -> bool {
        if let Ok(mut file) = File::open(SAVE_FILE) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(data) = serde_json::from_str::<SaveData>(&contents) {
                    self.seeds = data.seeds;
                    self.potatoes = data.potatoes;
                    self.farmer.grid_x = data.farmer_grid_x.min(GRID - 1);
                    self.farmer.grid_z = data.farmer_grid_z.min(GRID - 1);
                    self.farmer.position = Self::cell_center(self.farmer.grid_x, self.farmer.grid_z);

                    for (gx, row) in data.field.iter().enumerate().take(GRID) {
                        for (gz, cell) in row.iter().enumerate().take(GRID) {
                            self.field[gx][gz] = CellState::from(*cell);
                        }
                    }
                    self.set_msg("Game Loaded from savegame.json!");
                    return true;
                }
            }
        }
        self.set_msg("No save file found!");
        false
    }

    pub fn spawn_dirt(&mut self, pos: Vec3) {
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

    pub fn spawn_sparkles(&mut self, pos: Vec3) {
        for _ in 0..25 {
            let life = rand::gen_range(0.8, 1.6);
            let colors = [
                Color::from_rgba(255, 215, 0, 255),
                Color::from_rgba(255, 165, 0, 255),
                Color::from_rgba(100, 220, 100, 255),
                Color::from_rgba(255, 255, 180, 255),
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

    pub fn plow_cell(&mut self, gx: usize, gz: usize) {
        if self.field[gx][gz] == CellState::Grass {
            self.field[gx][gz] = CellState::Plowed;
            self.spawn_dirt(Self::cell_center(gx, gz));
        }
    }

    pub fn plant_cell(&mut self, gx: usize, gz: usize) -> bool {
        if self.seeds == 0 {
            self.set_msg("No seeds left! Trade potatoes at the Market.");
            return false;
        }

        if self.field[gx][gz] == CellState::Plowed {
            self.field[gx][gz] = CellState::Planted { growth: 0.0 };
            self.seeds -= 1;
            return true;
        }

        false
    }

    pub fn harvest_cell(&mut self, gx: usize, gz: usize) -> bool {
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

    pub fn convert_potatoes(&mut self) -> bool {
        if self.potatoes == 0 {
            self.set_msg("No potatoes to trade! Harvest mature potatoes first.");
            return false;
        }

        let converted = self.potatoes * POTATO_TO_SEED;
        let count = self.potatoes;
        self.seeds += converted;
        self.potatoes = 0;
        let target_pos = self.active_market_pos();
        self.spawn_sparkles(target_pos + vec3(0.0, 1.2, 0.0));
        self.set_msg(&format!("Traded {} Potatoes for {} Seeds at Market!", count, converted));
        true
    }

    pub fn distance_to_cell_center(&self) -> f32 {
        let target = Self::cell_center(self.farmer.grid_x, self.farmer.grid_z);
        self.farmer.position.distance(target)
    }

    pub fn handle_movement_input(&mut self, dt: f32) {
        let mut move_dir = Vec3::ZERO;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_dir.z -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_dir.z += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_dir.x += 1.0;
        }

        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.farmer.facing = move_dir.x.atan2(move_dir.z);

            let new_pos = self.farmer.position + move_dir * (MOVE_SPEED * dt);

            // 1. INVISIBLE MAP WALL BOUNDARY COLLISION
            let clamped_x = new_pos.x.clamp(MAP_LIMIT_X_MIN, MAP_LIMIT_X_MAX);
            let clamped_z = new_pos.z.clamp(MAP_LIMIT_Z_MIN, MAP_LIMIT_Z_MAX);
            let mut final_pos = vec3(clamped_x, 0.0, clamped_z);

            // 2. WATER RIVER & SHACK BRIDGE COLLISION
            // If trying to walk into the river section (x between RIVER_X_MIN and RIVER_X_MAX)
            let inside_river_x = final_pos.x > RIVER_X_MIN && final_pos.x < RIVER_X_MAX;
            let on_bridge = (final_pos.z - BRIDGE_Z_CENTER).abs() < BRIDGE_Z_HALF_WIDTH;

            if inside_river_x && !on_bridge {
                // Block player from stepping into water without using the bridge!
                if self.farmer.position.x <= RIVER_X_MIN {
                    final_pos.x = RIVER_X_MIN;
                } else if self.farmer.position.x >= RIVER_X_MAX {
                    final_pos.x = RIVER_X_MAX;
                } else if self.farmer.position.z < BRIDGE_Z_CENTER - BRIDGE_Z_HALF_WIDTH {
                    final_pos.z = BRIDGE_Z_CENTER - BRIDGE_Z_HALF_WIDTH;
                } else if self.farmer.position.z > BRIDGE_Z_CENTER + BRIDGE_Z_HALF_WIDTH {
                    final_pos.z = BRIDGE_Z_CENTER + BRIDGE_Z_HALF_WIDTH;
                }
            }

            self.farmer.position = final_pos;

            // Sync grid tile indices if inside crop field
            let field_rel_x = self.farmer.position.x + FIELD_HALF;
            let field_rel_z = self.farmer.position.z + FIELD_HALF;
            if field_rel_x >= 0.0 && field_rel_x < GRID as f32 * CELL &&
               field_rel_z >= 0.0 && field_rel_z < GRID as f32 * CELL {
                self.farmer.grid_x = (field_rel_x / CELL).floor() as usize;
                self.farmer.grid_z = (field_rel_z / CELL).floor() as usize;
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);
        self.farmer.step_cooldown = (self.farmer.step_cooldown - dt).max(0.0);
        self.msg_timer = (self.msg_timer - dt).max(0.0);

        if is_key_pressed(KeyCode::F5) || is_key_pressed(KeyCode::K) {
            self.save_game();
        }
        if is_key_pressed(KeyCode::F9) || is_key_pressed(KeyCode::L) {
            self.load_game();
        }

        self.handle_movement_input(dt);

        self.farmer.plowing = is_key_down(KeyCode::Space);

        if self.farmer.plowing && self.distance_to_cell_center() < 0.5 {
            self.plow_cell(self.farmer.grid_x, self.farmer.grid_z);
        }

        if is_key_pressed(KeyCode::E) && self.action_cooldown <= 0.0 {
            if self.near_market() {
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

        // Smooth Camera Follow
        let desired_target = self.farmer.position + vec3(0.0, 0.8, 0.0);
        let t = 1.0 - (-CAM_SMOOTH * dt).exp();
        self.camera.target = self.camera.target.lerp(desired_target, t);
        self.camera.position = self.camera.target + CAM_OFFSET;

        // Crop Growth
        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                if let CellState::Planted { growth } = cell {
                    *growth = (*growth + dt / GROW_TIME).min(1.0);
                }
            }
        }

        // Particle Physics
        for particle in self.dirt.iter_mut() {
            particle.velocity.y -= 12.0 * dt;
            particle.position += particle.velocity * dt;
            particle.life -= dt;
        }
        self.dirt.retain(|p| p.life > 0.0 && p.position.y > -0.5);

        for sparkle in self.sparkles.iter_mut() {
            sparkle.velocity.y -= 2.0 * dt;
            sparkle.position += sparkle.velocity * dt;
            sparkle.life -= dt;
        }
        self.sparkles.retain(|s| s.life > 0.0);

        // 3. B2 BOMBER & FIGHTER JET AIR COMBAT EVENT (Every 60 Seconds)
        self.air_event.timer += dt;
        if self.air_event.timer >= 60.0 {
            self.air_event.timer = 0.0;
            self.air_event.active = true;
            self.air_event.fly_time = 0.0;
            self.set_msg("AIR RAID INCOMING! B-2 Stealth Bomber & Fighter Jets Overhead!");
        }

        if self.air_event.active {
            self.air_event.fly_time += dt * 0.08; // ~12.5s flyby across sky

            let progress = self.air_event.fly_time;
            let start_x = -120.0;
            let end_x = 120.0;

            let cur_x = start_x + (end_x - start_x) * progress;
            let sky_y = 22.0;

            // B2 Bomber position
            self.air_event.bomber_pos = vec3(cur_x, sky_y, -10.0);

            // Fighter Jets pursuing B2 Bomber from behind
            self.air_event.jet1_pos = vec3(cur_x - 18.0, sky_y + 2.5, -4.0);
            self.air_event.jet2_pos = vec3(cur_x - 24.0, sky_y - 2.0, -16.0);

            // Fighter Jets fire tracer bullets towards the B2 Bomber!
            if rand::gen_range(0.0, 1.0) < 0.4 {
                let muzzle1 = self.air_event.jet1_pos + vec3(3.0, 0.0, 0.0);
                let dir1 = (self.air_event.bomber_pos - muzzle1).normalize();
                self.air_event.bullets.push(BulletParticle {
                    position: muzzle1,
                    velocity: dir1 * 60.0,
                    life: 0.8,
                });
            }
            if rand::gen_range(0.0, 1.0) < 0.4 {
                let muzzle2 = self.air_event.jet2_pos + vec3(3.0, 0.0, 0.0);
                let dir2 = (self.air_event.bomber_pos - muzzle2).normalize();
                self.air_event.bullets.push(BulletParticle {
                    position: muzzle2,
                    velocity: dir2 * 60.0,
                    life: 0.8,
                });
            }

            if progress >= 1.0 {
                self.air_event.active = false;
            }
        }

        // Bullet physics & retention
        for bullet in self.air_event.bullets.iter_mut() {
            bullet.position += bullet.velocity * dt;
            bullet.life -= dt;
        }
        self.air_event.bullets.retain(|b| b.life > 0.0);
    }
}
