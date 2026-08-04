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
    pub houses: Vec<HouseBounds>,
    // New Tycoon & Defense Features
    pub turrets_unlocked: bool,
    pub turrets: Vec<Turret>,
    pub children: Vec<ThiefChild>,
    pub steal_timer: f32,
    pub turret_bullets: Vec<BulletParticle>,
}

impl Game {
    pub fn new() -> Self {
        let start_x = (GRID / 2) as i32;
        let start_z = (GRID / 2) as i32;
        let start_pos = Self::grid_to_world(start_x, start_z);
        let cam_target = start_pos + vec3(0.0, 0.8, 0.0);

        let mut houses = Vec::new();
        Self::generate_houses(&mut houses);

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
                timer: 50.0,
                fly_time: 0.0,
                bomber_pos: Vec3::ZERO,
                jet1_pos: Vec3::ZERO,
                jet2_pos: Vec3::ZERO,
                bullets: Vec::new(),
            },
            houses,
            turrets_unlocked: false,
            turrets: Vec::new(),
            children: Vec::new(),
            steal_timer: 0.0,
            turret_bullets: Vec::new(),
        };

        if std::path::Path::new(SAVE_FILE).exists() {
            game.load_game();
            game.set_msg("Welcome back! Auto-loaded saved game.");
        }

        game
    }

    pub fn setup_turrets(&mut self) {
        self.turrets.clear();
        // 4 Corner Defense Turrets around the Field
        self.turrets.push(Turret { position: vec3(-FIELD_HALF - 1.5, 0.0, -FIELD_HALF - 1.5), fire_cooldown: 0.0 });
        self.turrets.push(Turret { position: vec3(FIELD_HALF + 1.5, 0.0, -FIELD_HALF - 1.5), fire_cooldown: 0.0 });
        self.turrets.push(Turret { position: vec3(-FIELD_HALF - 1.5, 0.0, FIELD_HALF + 1.5), fire_cooldown: 0.0 });
        self.turrets.push(Turret { position: vec3(FIELD_HALF + 1.5, 0.0, FIELD_HALF + 1.5), fire_cooldown: 0.0 });
    }

    pub fn generate_houses(houses: &mut Vec<HouseBounds>) {
        let mut idx = 0;
        let mut add_h = |center: Vec3, size: Vec2| {
            houses.push(HouseBounds {
                center,
                min_x: center.x - size.x / 2.0 - 0.4,
                max_x: center.x + size.x / 2.0 + 0.4,
                min_z: center.z - size.y / 2.0 - 0.4,
                max_z: center.z + size.y / 2.0 + 0.4,
                style: idx % 4,
            });
            idx += 1;
        };

        // North Border
        for &x in &[-24.0, -16.0, -8.0, 0.0, 8.0, 16.0, 24.0] {
            add_h(vec3(x, 0.0, -FIELD_HALF - 5.0), vec2(3.6, 3.6));
        }
        // East Border
        for &z in &[-18.0, -10.0, 10.0, 18.0] {
            add_h(vec3(FIELD_HALF + 5.0, 0.0, z), vec2(3.6, 3.6));
        }
        // South Border
        for &x in &[24.0, 16.0, 8.0, 0.0, -8.0, -16.0, -24.0] {
            add_h(vec3(x, 0.0, FIELD_HALF + 5.0), vec2(3.6, 3.6));
        }
        // West Border
        for &z in &[18.0, 10.0, -10.0, -18.0] {
            add_h(vec3(-FIELD_HALF - 5.0, 0.0, z), vec2(3.6, 3.6));
        }
    }

    pub fn grid_to_world(gx: i32, gz: i32) -> Vec3 {
        vec3(
            -FIELD_HALF + (gx as f32 * CELL) + (CELL / 2.0),
            0.0,
            -FIELD_HALF + (gz as f32 * CELL) + (CELL / 2.0),
        )
    }

    pub fn cell_center(gx: usize, gz: usize) -> Vec3 {
        Self::grid_to_world(gx as i32, gz as i32)
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
            turrets_unlocked: self.turrets_unlocked,
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
                    self.farmer.grid_x = data.farmer_grid_x;
                    self.farmer.grid_z = data.farmer_grid_z;
                    self.farmer.position = Self::grid_to_world(self.farmer.grid_x, self.farmer.grid_z);
                    self.turrets_unlocked = data.turrets_unlocked;
                    if self.turrets_unlocked {
                        self.setup_turrets();
                    }

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
        for _ in 0..6 {
            let shade = (60.0 + rand::gen_range(0.0, 40.0)) as u8;
            self.dirt.push(DirtParticle {
                position: pos + vec3(rand::gen_range(-0.4, 0.4), 0.15, rand::gen_range(-0.4, 0.4)),
                velocity: vec3(
                    rand::gen_range(-2.0, 2.0),
                    rand::gen_range(3.0, 6.0),
                    rand::gen_range(-2.0, 2.0),
                ),
                life: rand::gen_range(0.5, 0.9),
                color: Color::from_rgba(shade, shade / 2 + 10, 20, 255),
            });
        }
    }

    pub fn spawn_sparkles(&mut self, pos: Vec3) {
        for _ in 0..16 {
            let life = rand::gen_range(0.6, 1.2);
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
                    rand::gen_range(-1.8, 1.8),
                    rand::gen_range(2.0, 4.5),
                    rand::gen_range(-1.8, 1.8),
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

    pub fn buy_turret_upgrade(&mut self) -> bool {
        if self.turrets_unlocked {
            self.set_msg("Defensive Guard Turrets already installed!");
            return false;
        }
        if self.potatoes < TURRET_UPGRADE_COST {
            self.set_msg(&format!("Need 150 Potatoes to purchase Turrets! (Have {})", self.potatoes));
            return false;
        }

        self.potatoes -= TURRET_UPGRADE_COST;
        self.turrets_unlocked = true;
        self.setup_turrets();
        let m_pos = self.active_market_pos();
        self.spawn_sparkles(m_pos + vec3(0.0, 1.5, 0.0));
        self.set_msg("UNLOCKED! 4 Corner Automated Gun Turrets installed on Farm!");
        true
    }

    // Checking if target position hits any house OR market solid bounding box
    pub fn hits_solid_obstacle(&self, target_pos: Vec3) -> bool {
        // 1. House Solid Bounds
        for h in &self.houses {
            if target_pos.x >= h.min_x && target_pos.x <= h.max_x &&
               target_pos.z >= h.min_z && target_pos.z <= h.max_z {
                return true;
            }
        }

        // 2. West & East Market Solid Bounds (3.8 x 3.8 box around market center)
        let w_min_x = WEST_MARKET_POS.x - 2.0;
        let w_max_x = WEST_MARKET_POS.x + 2.0;
        let w_min_z = WEST_MARKET_POS.z - 2.0;
        let w_max_z = WEST_MARKET_POS.z + 2.0;
        if target_pos.x >= w_min_x && target_pos.x <= w_max_x &&
           target_pos.z >= w_min_z && target_pos.z <= w_max_z {
            return true;
        }

        let e_min_x = EAST_MARKET_POS.x - 2.0;
        let e_max_x = EAST_MARKET_POS.x + 2.0;
        let e_min_z = EAST_MARKET_POS.z - 2.0;
        let e_max_z = EAST_MARKET_POS.z + 2.0;
        if target_pos.x >= e_min_x && target_pos.x <= e_max_x &&
           target_pos.z >= e_min_z && target_pos.z <= e_max_z {
            return true;
        }

        false
    }

    pub fn try_step(&mut self, dx: i32, dz: i32) -> bool {
        let nx = self.farmer.grid_x + dx;
        let nz = self.farmer.grid_z + dz;
        let target_pos = Self::grid_to_world(nx, nz);

        // 1. INVISIBLE MAP OUTSIDE WALL BOUNDS
        if target_pos.x < MAP_LIMIT_X_MIN || target_pos.x > MAP_LIMIT_X_MAX ||
           target_pos.z < MAP_LIMIT_Z_MIN || target_pos.z > MAP_LIMIT_Z_MAX {
            return false;
        }

        // 2. WATER RIVER & WOODEN BRIDGE COLLISION
        let inside_river = target_pos.x > RIVER_X_MIN && target_pos.x < RIVER_X_MAX;
        let on_bridge = (target_pos.z - BRIDGE_Z_CENTER).abs() < BRIDGE_Z_HALF_WIDTH;
        if inside_river && !on_bridge {
            return false;
        }

        // 3. HOUSE & MARKET SOLID BORDER COLLISION
        if self.hits_solid_obstacle(target_pos) {
            return false;
        }

        self.farmer.grid_x = nx;
        self.farmer.grid_z = nz;
        self.farmer.facing = (dx as f32).atan2(dz as f32);
        self.farmer.step_cooldown = STEP_REPEAT;

        true
    }

    pub fn handle_movement_input(&mut self) {
        let target = Self::grid_to_world(self.farmer.grid_x, self.farmer.grid_z);
        if self.farmer.position.distance(target) > 0.15 {
            return;
        }

        if self.farmer.step_cooldown > 0.0 {
            return;
        }

        let mut dx = 0;
        let mut dz = 0;

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dz -= 1;
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dz += 1;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1;
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1;
        }

        if dx != 0 || dz != 0 {
            self.try_step(dx, dz);
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

        self.handle_movement_input();

        // Interpolate farmer position
        let target = Self::grid_to_world(self.farmer.grid_x, self.farmer.grid_z);
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

        // Farming actions
        let is_in_field = self.farmer.grid_x >= 0 && self.farmer.grid_x < GRID as i32 &&
                         self.farmer.grid_z >= 0 && self.farmer.grid_z < GRID as i32;

        if is_in_field {
            let gx = self.farmer.grid_x as usize;
            let gz = self.farmer.grid_z as usize;

            if self.farmer.plowing {
                self.plow_cell(gx, gz);
            }

            if is_key_pressed(KeyCode::E) && self.action_cooldown <= 0.0 {
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

        // Market Interaction Hotkeys: [E] Trade Potatoes -> Seeds, [T] Upgrade Turrets (150 Potatoes)
        if self.near_market() && self.action_cooldown <= 0.0 {
            if is_key_pressed(KeyCode::E) {
                self.convert_potatoes();
                self.action_cooldown = 0.4;
            } else if is_key_pressed(KeyCode::T) {
                self.buy_turret_upgrade();
                self.action_cooldown = 0.4;
            }
        }

        // Smooth Camera
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

        // --- NEW THIEF CHILDREN EVENT (Only applicable once player reaches 150 potatoes!) ---
        if self.potatoes >= 150 || !self.children.is_empty() {
            self.steal_timer += dt;

            // Every 6 seconds, spawn a group of thief children to raid 5 potato fields
            if self.steal_timer >= 6.0 {
                self.steal_timer = 0.0;

                // Find 5 random mature/planted potato fields to target
                let mut target_cells = Vec::new();
                for gx in 0..GRID {
                    for gz in 0..GRID {
                        if matches!(self.field[gx][gz], CellState::Planted { .. }) {
                            target_cells.push((gx, gz));
                        }
                    }
                }

                // Spawn up to 5 thief children from perimeter village borders
                let spawn_count = 5.min(target_cells.len());
                for i in 0..spawn_count {
                    let (gx, gz) = target_cells[i];
                    let spawn_pos = vec3(
                        if i % 2 == 0 { -24.0 } else { 24.0 },
                        0.0,
                        if i < 2 { -24.0 } else { 24.0 },
                    );

                    self.children.push(ThiefChild {
                        position: spawn_pos,
                        target_cell: Some((gx, gz)),
                        speed: 7.5,
                        fleeing: false,
                        alive: true,
                    });
                }

                if spawn_count > 0 && self.msg_timer <= 0.0 {
                    self.set_msg("WARNING! Thief Children raiding your Potato Fields!");
                }
            }
        }

        // Update Thief Children AI
        for child in self.children.iter_mut() {
            if !child.alive {
                continue;
            }

            if !child.fleeing {
                if let Some((gx, gz)) = child.target_cell {
                    let target_pos = Self::cell_center(gx, gz);
                    let to_target = target_pos - child.position;
                    let dist = to_target.length();

                    if dist > 0.4 {
                        child.position += to_target.normalize() * (child.speed * dt);
                    } else {
                        // Reached potato field - Steal crop!
                        child.fleeing = true;
                        child.target_cell = None;
                    }
                } else {
                    child.fleeing = true;
                }
            } else {
                // Flee back towards village border
                let flee_dir = (child.position - Vec3::ZERO).normalize();
                child.position += flee_dir * (child.speed * dt);
            }
        }

        // Apply crop steals once children reach target or escape
        for child in self.children.iter_mut() {
            if child.fleeing && child.alive {
                // Steal the target cell if not stolen yet
                if let Some((gx, gz)) = child.target_cell.take() {
                    self.field[gx][gz] = CellState::Grass;
                }
            }
        }

        // Remove escaped or dead children
        self.children.retain(|c| c.alive && c.position.length() < 40.0);

        // --- AUTOMATED DEFENSE TURRETS ENGINE ---
        if self.turrets_unlocked {
            for turret in self.turrets.iter_mut() {
                turret.fire_cooldown = (turret.fire_cooldown - dt).max(0.0);

                if turret.fire_cooldown <= 0.0 {
                    // Find nearest active thief child within range (18 units)
                    let t_pos = turret.position + vec3(0.0, 1.2, 0.0);
                    let mut nearest_idx: Option<usize> = None;
                    let mut min_dist = 18.0;

                    for (idx, child) in self.children.iter().enumerate() {
                        if child.alive {
                            let d = t_pos.distance(child.position);
                            if d < min_dist {
                                min_dist = d;
                                nearest_idx = Some(idx);
                            }
                        }
                    }

                    if let Some(idx) = nearest_idx {
                        let target_child_pos = self.children[idx].position + vec3(0.0, 0.6, 0.0);
                        let dir = (target_child_pos - t_pos).normalize();

                        // Fire high-speed turret laser bullet
                        self.turret_bullets.push(BulletParticle {
                            position: t_pos,
                            velocity: dir * 45.0,
                            life: 0.5,
                        });

                        turret.fire_cooldown = 0.25; // Rapid fire
                    }
                }
            }

            // Update Turret Bullets & Collision with Thief Children
            for bullet in self.turret_bullets.iter_mut() {
                bullet.position += bullet.velocity * dt;
                bullet.life -= dt;

                // Check bullet collision with thief children
                for child in self.children.iter_mut() {
                    if child.alive && bullet.position.distance(child.position + vec3(0.0, 0.6, 0.0)) < 0.8 {
                        child.alive = false;
                        bullet.life = 0.0;
                        break;
                    }
                }
            }
            self.turret_bullets.retain(|b| b.life > 0.0);
        }

        // Air Event (B-2 Stealth Bomber Shootout every 60s)
        self.air_event.timer += dt;
        if self.air_event.timer >= 60.0 {
            self.air_event.timer = 0.0;
            self.air_event.active = true;
            self.air_event.fly_time = 0.0;
            self.set_msg("AIR RAID INCOMING! B-2 Stealth Bomber & Fighter Jets Overhead!");
        }

        if self.air_event.active {
            self.air_event.fly_time += dt * 0.08;
            let progress = self.air_event.fly_time;
            let cur_x = -120.0 + 240.0 * progress;
            let sky_y = 22.0;

            self.air_event.bomber_pos = vec3(cur_x, sky_y, -10.0);
            self.air_event.jet1_pos = vec3(cur_x - 18.0, sky_y + 2.5, -4.0);
            self.air_event.jet2_pos = vec3(cur_x - 24.0, sky_y - 2.0, -16.0);

            if rand::gen_range(0.0, 1.0) < 0.25 {
                let muzzle1 = self.air_event.jet1_pos + vec3(3.0, 0.0, 0.0);
                let dir1 = (self.air_event.bomber_pos - muzzle1).normalize();
                self.air_event.bullets.push(BulletParticle {
                    position: muzzle1,
                    velocity: dir1 * 60.0,
                    life: 0.8,
                });
            }

            if progress >= 1.0 {
                self.air_event.active = false;
            }
        }

        for bullet in self.air_event.bullets.iter_mut() {
            bullet.position += bullet.velocity * dt;
            bullet.life -= dt;
        }
        self.air_event.bullets.retain(|b| b.life > 0.0);
    }
}
