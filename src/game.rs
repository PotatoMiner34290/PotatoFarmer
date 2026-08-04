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
        };

        // Auto-load saved game if it exists
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

    pub fn try_step(&mut self, dx: i32, dz: i32) -> bool {
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

    pub fn handle_movement_input(&mut self) {
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

    pub fn update(&mut self, dt: f32) {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);
        self.farmer.step_cooldown = (self.farmer.step_cooldown - dt).max(0.0);
        self.msg_timer = (self.msg_timer - dt).max(0.0);

        // Save & Load Hotkeys
        if is_key_pressed(KeyCode::F5) || is_key_pressed(KeyCode::K) {
            self.save_game();
        }
        if is_key_pressed(KeyCode::F9) || is_key_pressed(KeyCode::L) {
            self.load_game();
        }

        self.handle_movement_input();

        // Constant velocity movement towards target cell
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
