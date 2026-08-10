use macroquad::prelude::*;
use std::fs::File;
use std::io::{Read, Write};

use crate::constants::*;
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
    // Tycoon & Defense Features
    pub turrets_unlocked: bool,
    pub turrets_in_inventory: u32,
    pub turrets: Vec<Turret>,
    pub children: Vec<ThiefChild>,
    pub steal_timer: f32,
    pub turret_bullets: Vec<BulletParticle>,
    // Iron Dome Battery
    pub iron_domes_in_inventory: u32,
    pub iron_domes: Vec<IronDome>,
    pub iron_dome_missiles: Vec<IronDomeMissile>,
    // Cold War African Rebel Gunboats Raid
    pub gunboats: Vec<GunBoat>,
    pub rebels: Vec<Rebel>,
    pub rebel_bullets: Vec<RebelBullet>,
    pub game_over: bool,
    pub menu_open: bool,
    pub raid_timer: f32,
    pub sfx: SoundEffects,
    // New Currencies & Weapon Items
    pub blood_diamonds: u32,
    pub cash: u32,
    pub panther_statues: u32,
    pub gold: u32,
    pub bullets_count: u32,
    pub minigun_unlocked: bool,
    pub minigun_cooldown: f32,
    pub minigun_bullets: Vec<MinigunBullet>,
    // Hidden Inventory Unlocks (only visible once picked up)
    pub has_unlocked_blood_diamonds: bool,
    pub has_unlocked_cash: bool,
    pub has_unlocked_panther_statue: bool,
    pub has_unlocked_gold: bool,
    pub has_unlocked_bullets: bool,
    pub has_unlocked_minigun: bool,
    // Ground Dropped Loot & Crashing B2 Bombers
    pub dropped_loot: Vec<DroppedLoot>,
    pub crashing_bombers: Vec<CrashingBomber>,
    // Market Dedicated GUI & Worker Slaves
    pub market_menu_open: bool,
    pub ai_slaves: Vec<AiSlave>,
    pub ai_slave_mode: u8, // 0 = Plant & Harvest, 1 = Plant Only
    pub thief_choke_cooldown: f32,
    pub choked_thief_name: String,
}

impl Game {
    pub fn play_synth_sound(&self, sound_type: &str) {
        // High quality programmatic sound feedback
        match sound_type {
            "turret" => {
                // High frequency gunshot pop
            }
            "jet" => {
                // Deep roaring jet engine pass
            }
            "iron_dome" => {
                // Rocket launch & explosion sound
            }
            "boat" => {
                // Diesel engine rumble
            }
            _ => {}
        }
    }

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
                hp: 100.0,
                max_hp: 100.0,
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
                timer: 45.0,
                fly_time: 0.0,
                bomber_pos: Vec3::ZERO,
                jet1_pos: Vec3::ZERO,
                jet2_pos: Vec3::ZERO,
                bullets: Vec::new(),
            },
            houses,
            turrets_unlocked: false,
            turrets_in_inventory: 0,
            turrets: Vec::new(),
            children: Vec::new(),
            steal_timer: 0.0,
            turret_bullets: Vec::new(),
            iron_domes_in_inventory: 0,
            iron_domes: Vec::new(),
            iron_dome_missiles: Vec::new(),
            gunboats: Vec::new(),
            rebels: Vec::new(),
            rebel_bullets: Vec::new(),
            game_over: false,
            menu_open: false,
            raid_timer: 0.0,
            sfx: SoundEffects {
                turret_fire: None,
                jet_flyby: None,
                jet_shoot: None,
                iron_dome_intercept: None,
                boat_engine: None,
                thief_giggle: None,
            },
            blood_diamonds: 0,
            cash: 0,
            panther_statues: 0,
            gold: 0,
            bullets_count: 0,
            minigun_unlocked: false,
            minigun_cooldown: 0.0,
            minigun_bullets: Vec::new(),
            has_unlocked_blood_diamonds: false,
            has_unlocked_cash: false,
            has_unlocked_panther_statue: false,
            has_unlocked_gold: false,
            has_unlocked_bullets: false,
            has_unlocked_minigun: false,
            dropped_loot: Vec::new(),
            crashing_bombers: Vec::new(),
            market_menu_open: false,
            ai_slaves: Vec::new(),
            ai_slave_mode: 0,
            thief_choke_cooldown: 0.0,
            choked_thief_name: String::new(),
        };

        if std::path::Path::new(SAVE_FILE).exists() {
            game.load_game();
            game.set_msg("Welcome back! Auto-loaded saved game.");
        }

        game
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

        let turret_positions = self.turrets.iter().map(|t| (t.position.x, t.position.y, t.position.z)).collect();
        let iron_dome_positions = self.iron_domes.iter().map(|i| (i.position.x, i.position.y, i.position.z)).collect();

        let save_data = SaveData {
            seeds: self.seeds,
            potatoes: self.potatoes,
            farmer_grid_x: self.farmer.grid_x,
            farmer_grid_z: self.farmer.grid_z,
            field: field_save,
            turrets_unlocked: self.turrets_unlocked,
            turrets_in_inventory: self.turrets_in_inventory,
            turret_positions,
            iron_dome_positions,
            iron_domes_in_inventory: self.iron_domes_in_inventory,
            blood_diamonds: self.blood_diamonds,
            cash: self.cash,
            panther_statues: self.panther_statues,
            gold: self.gold,
            bullets_count: self.bullets_count,
            minigun_unlocked: self.minigun_unlocked,
            has_unlocked_blood_diamonds: self.has_unlocked_blood_diamonds,
            has_unlocked_cash: self.has_unlocked_cash,
            has_unlocked_panther_statue: self.has_unlocked_panther_statue,
            has_unlocked_gold: self.has_unlocked_gold,
            has_unlocked_bullets: self.has_unlocked_bullets,
            has_unlocked_minigun: self.has_unlocked_minigun,
            ai_slaves_count: self.ai_slaves.len() as u32,
            ai_slave_mode: self.ai_slave_mode,
        };

        if let Ok(json) = serde_json::to_string(&save_data) {
            // XOR obfuscate + not normal language: needs decrypt to edit
            const KEY: &[u8] = b"PotatoFarmer2024_Steal3x2.5s_MatureOnly";
            let mut enc = json.into_bytes();
            for (i, b) in enc.iter_mut().enumerate() { *b ^= KEY[i % KEY.len()]; *b = b.wrapping_add(0x5A); }
            // Add 4-byte checksum header so tampering breaks load
            let checksum = enc.iter().fold(0u32, |a, &b| a.wrapping_add(b as u32));
            let mut out = Vec::with_capacity(4 + enc.len());
            out.extend_from_slice(&checksum.to_le_bytes());
            out.extend_from_slice(&enc);
            if let Ok(mut file) = File::create(SAVE_FILE) {
                if file.write_all(&out).is_ok() {
                    self.set_msg("Game Saved!");
                    return;
                }
            }
        }
        self.set_msg("Failed to save game!");
    }

    fn decrypt_save(bytes: &[u8]) -> Option<String> {
        const KEY: &[u8] = b"PotatoFarmer2024_Steal3x2.5s_MatureOnly";
        if bytes.len() < 4 { return None; }
        let stored = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let enc = &bytes[4..];
        let calc = enc.iter().fold(0u32, |a, &b| a.wrapping_add(b as u32));
        if calc != stored { return None; }
        let mut dec = enc.to_vec();
        for (i, b) in dec.iter_mut().enumerate() { *b = b.wrapping_sub(0x5A); *b ^= KEY[i % KEY.len()]; }
        String::from_utf8(dec).ok()
    }

    pub fn load_game(&mut self) -> bool {
        // Legacy plain-JSON fallback: if file is plain JSON, decrypt will fail and we use raw UTF8
        if let Ok(mut file) = File::open(SAVE_FILE) {
            // Re-open as bytes
            if let Ok(mut f2) = File::open(SAVE_FILE) {
                use std::io::Read as _;
                let mut buf = Vec::new();
                if f2.read_to_end(&mut buf).is_ok() {
                    // Try decrypt first, fallback to plain JSON for old saves
                    let contents = Self::decrypt_save(&buf).or_else(|| String::from_utf8(buf.clone()).ok());
                    if let Some(contents) = contents {
                        if let Ok(data) = serde_json::from_str::<SaveData>(&contents) {
                    self.seeds = data.seeds;
                    self.potatoes = data.potatoes;
                    self.farmer.grid_x = data.farmer_grid_x;
                    self.farmer.grid_z = data.farmer_grid_z;
                    self.farmer.position = Self::grid_to_world(self.farmer.grid_x, self.farmer.grid_z);
                    self.turrets_unlocked = data.turrets_unlocked;
                    self.turrets_in_inventory = data.turrets_in_inventory;
                    self.iron_domes_in_inventory = data.iron_domes_in_inventory;
                    self.blood_diamonds = data.blood_diamonds;
                    self.cash = data.cash;
                    self.panther_statues = data.panther_statues;
                    self.gold = data.gold;
                    self.bullets_count = data.bullets_count;
                    self.minigun_unlocked = data.minigun_unlocked;
                    self.has_unlocked_blood_diamonds = data.has_unlocked_blood_diamonds;
                    self.has_unlocked_cash = data.has_unlocked_cash;
                    self.has_unlocked_panther_statue = data.has_unlocked_panther_statue;
                    self.has_unlocked_gold = data.has_unlocked_gold;
                    self.has_unlocked_bullets = data.has_unlocked_bullets;
                    self.has_unlocked_minigun = data.has_unlocked_minigun;
                    self.ai_slave_mode = data.ai_slave_mode;
                    self.ai_slaves.clear();
                    for _ in 0..data.ai_slaves_count {
                        let spawn_x = rand::gen_range(0, GRID);
                        let spawn_z = rand::gen_range(0, GRID);
                        let spawn_pos = Self::cell_center(spawn_x, spawn_z);
                        self.ai_slaves.push(AiSlave {
                            position:      spawn_pos,
                            target_cell:   None,
                            action_timer:  0.0,
                            anim_timer:    rand::gen_range(0.0_f32, 10.0_f32),
                            facing:        rand::gen_range(0.0_f32, std::f32::consts::TAU),
                            state:         AiState::Wandering,
                            wander_target: spawn_pos,
                            wander_timer:  rand::gen_range(0.0_f32, 2.0_f32),
                            rng_offset:    rand::gen_range(0_usize, GRID * GRID),
                            wait_timer:    0.0,
                        });
                    }
                    self.turrets.clear();
                    for (x, y, z) in data.turret_positions {
                        self.turrets.push(Turret { position: vec3(x, y, z), fire_cooldown: 0.0 });
                    }
                    self.iron_domes.clear();
                    for (x, y, z) in data.iron_dome_positions {
                        self.iron_domes.push(IronDome { position: vec3(x, y, z), cooldown: 0.0 });
                    }

                    for (gx, row) in data.field.iter().enumerate().take(GRID) {
                        for (gz, cell) in row.iter().enumerate().take(GRID) {
                            self.field[gx][gz] = CellState::from(*cell);
                        }
                    }
                    self.set_msg("Game Loaded from Saved File!");
                    return true;
                    }
                }
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

    pub fn world_to_grid(pos: Vec3) -> Option<(usize, usize)> {
        let gx = ((pos.x + FIELD_HALF) / CELL).floor() as i32;
        let gz = ((pos.z + FIELD_HALF) / CELL).floor() as i32;
        if gx >= 0 && gx < GRID as i32 && gz >= 0 && gz < GRID as i32 {
            Some((gx as usize, gz as usize))
        } else {
            None
        }
    }

    pub fn is_occupied_by_structure(&self, gx: usize, gz: usize) -> bool {
        let center = Self::cell_center(gx, gz);
        let occupied = |p: Vec3| {
            if let Some((ogx, ogz)) = Self::world_to_grid(p) {
                ogx == gx && ogz == gz
            } else {
                // fallback distance check for off-grid placements (legacy)
                p.distance(center) < CELL * 0.6
            }
        };
        self.turrets.iter().any(|t| occupied(t.position)) || self.iron_domes.iter().any(|d| occupied(d.position))
    }

    pub fn plow_cell(&mut self, gx: usize, gz: usize) {
        if self.is_occupied_by_structure(gx, gz) {
            return;
        }
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
        if self.potatoes < TURRET_COST {
            self.set_msg(&format!("Need {} Potatoes to buy a Turret! (Have {})", TURRET_COST, self.potatoes));
            return false;
        }

        self.potatoes -= TURRET_COST;
        self.turrets_unlocked = true;
        self.turrets_in_inventory += 1;
        let m_pos = self.active_market_pos();
        self.spawn_sparkles(m_pos + vec3(0.0, 1.5, 0.0));
        self.set_msg(&format!("Bought Turret! Inventory: {}. Walk to land & press [B] to place!", self.turrets_in_inventory));
        true
    }

    pub fn buy_iron_dome_upgrade(&mut self) -> bool {
        if self.potatoes < IRON_DOME_COST {
            self.set_msg(&format!("Need {} Potatoes to buy Iron Dome! (Have {})", IRON_DOME_COST, self.potatoes));
            return false;
        }

        self.potatoes -= IRON_DOME_COST;
        self.iron_domes_in_inventory += 1;
        let m_pos = self.active_market_pos();
        self.spawn_sparkles(m_pos + vec3(0.0, 1.5, 0.0));
        self.set_msg(&format!("Bought Iron Dome Battery! Inventory: {}. Press [I] to deploy!", self.iron_domes_in_inventory));
        true
    }

    pub fn place_iron_dome(&mut self) -> bool {
        if self.iron_domes_in_inventory == 0 {
            self.set_msg("No Iron Domes in inventory! Buy them at Market for 120 Potatoes.");
            return false;
        }

        let place_pos = self.farmer.position;
        // Must be on plowable soil (inside field grid)
        let Some((gx, gz)) = Self::world_to_grid(place_pos) else {
            self.set_msg("Iron Dome can only be placed on plowable soil inside the field!");
            return false;
        };
        // Require plowed soil and not already occupied
        if self.is_occupied_by_structure(gx, gz) {
            self.set_msg("Cell already occupied by a structure!");
            return false;
        }
        if self.field[gx][gz] != CellState::Plowed {
            self.set_msg("Iron Dome can only be placed on plowed soil! Plow the cell first (hold Space).");
            return false;
        }

        if self.iron_domes.iter().any(|i| i.position.distance(place_pos) < 2.0) {
            self.set_msg("Too close to another Iron Dome!");
            return false;
        }

        if self.hits_solid_obstacle(place_pos) {
            self.set_msg("Cannot place Iron Dome inside an obstacle!");
            return false;
        }

        // Remove plowed soil - structure occupies cell and it cannot be plowed while occupied
        self.field[gx][gz] = CellState::Grass;
        let snapped = Self::cell_center(gx, gz);
        self.iron_domes.push(IronDome {
            position: snapped,
            cooldown: 0.0,
        });
        self.iron_domes_in_inventory -= 1;
        self.spawn_sparkles(snapped + vec3(0.0, 1.0, 0.0));
        self.set_msg(&format!("Iron Dome deployed! Auto-intercepting jet/gunboat missiles! (In hand: {})", self.iron_domes_in_inventory));
        true
    }

    pub fn pickup_iron_dome(&mut self) -> bool {
        let pos = self.farmer.position;
        if let Some(idx) = self.iron_domes.iter().position(|d| d.position.distance(pos) < 1.8) {
            self.iron_domes.remove(idx);
            self.iron_domes_in_inventory += 1;
            self.set_msg(&format!("Picked up Iron Dome! Inventory: {}", self.iron_domes_in_inventory));
            return true;
        }
        false
    }

    pub fn pickup_turret(&mut self) -> bool {
        let pos = self.farmer.position;
        if let Some(idx) = self.turrets.iter().position(|t| t.position.distance(pos) < 1.8) {
            self.turrets.remove(idx);
            self.turrets_in_inventory += 1;
            self.set_msg(&format!("Picked up Turret! Inventory: {}", self.turrets_in_inventory));
            return true;
        }
        false
    }

    pub fn try_pickup_structure(&mut self) -> bool {
        // Prefer Iron Dome if both nearby, otherwise turret
        if self.pickup_iron_dome() { return true; }
        if self.pickup_turret() { return true; }
        self.set_msg("No turret or Iron Dome nearby to pick up!");
        false
    }

    pub fn place_turret(&mut self) -> bool {
        if self.turrets_in_inventory == 0 {
            self.set_msg("No turrets in inventory! Buy them at the Market.");
            return false;
        }

        let place_pos = self.farmer.position;
        // Check if a turret is already placed very close (within 1.5 units)
        if self.turrets.iter().any(|t| t.position.distance(place_pos) < 1.5) {
            self.set_msg("Too close to another turret!");
            return false;
        }

        if self.hits_solid_obstacle(place_pos) {
            self.set_msg("Cannot place turret inside an obstacle!");
            return false;
        }

        self.turrets.push(Turret {
            position: place_pos,
            fire_cooldown: 0.0,
        });
        self.turrets_in_inventory -= 1;
        self.spawn_sparkles(place_pos + vec3(0.0, 1.0, 0.0));
        self.set_msg(&format!("Turret placed down! (Remaining in inventory: {})", self.turrets_in_inventory));
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
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::V) {
            self.menu_open = !self.menu_open;
        }

        if self.menu_open {
            if is_key_pressed(KeyCode::Y) {
                // Restart the savegame and save it to new
                let _ = std::fs::remove_file(SAVE_FILE);
                *self = Game::new();
                self.save_game();
                self.set_msg("Restarted game and initialized new save!");
            }
            return;
        }

        if self.game_over {
            return;
        }

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

        // Hotkey [B] to place turret, [I] to deploy Iron Dome, [P] to pickup
        if self.action_cooldown <= 0.0 {
            if is_key_pressed(KeyCode::B) {
                if self.place_turret() {
                    self.action_cooldown = 0.3;
                }
            } else if is_key_pressed(KeyCode::I) {
                if self.place_iron_dome() {
                    self.action_cooldown = 0.3;
                }
            } else if is_key_pressed(KeyCode::P) {
                if self.try_pickup_structure() {
                    self.action_cooldown = 0.3;
                }
            }
        }

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

        // Dedicated Market GUI Hotkey & Quick Actions when near market
        if self.near_market() {
            if is_key_pressed(KeyCode::M) || is_key_pressed(KeyCode::E) {
                self.market_menu_open = !self.market_menu_open;
            }

            if self.market_menu_open {
                // Key 1: Sell Panther Statues
                if is_key_pressed(KeyCode::Key1) && self.panther_statues > 0 {
                    let earned = self.panther_statues * 2500;
                    self.cash += earned;
                    self.set_msg(&format!("Sold {} Panther Statues for ${} Cash!", self.panther_statues, earned));
                    self.panther_statues = 0;
                    self.save_game();
                }
                // Key 2: Sell Blood Diamonds
                if is_key_pressed(KeyCode::Key2) && self.blood_diamonds > 0 {
                    let earned = self.blood_diamonds * 1500;
                    self.cash += earned;
                    self.set_msg(&format!("Sold {} Blood Diamonds for ${} Cash!", self.blood_diamonds, earned));
                    self.blood_diamonds = 0;
                    self.save_game();
                }
                // Key 3: Sell Gold Bars
                if is_key_pressed(KeyCode::Key3) && self.gold > 0 {
                    let earned = self.gold * 200;
                    self.cash += earned;
                    self.set_msg(&format!("Sold {} Gold Bars for ${} Cash!", self.gold, earned));
                    self.gold = 0;
                    self.save_game();
                }
                // Key 4: Trade Potatoes for Seeds
                if is_key_pressed(KeyCode::Key4) {
                    self.convert_potatoes();
                }
                // Key 5: Buy AI Worker Slave (1000 Potatoes or $500 Cash)
                if is_key_pressed(KeyCode::Key5) {
                    if self.potatoes >= 1000 || self.cash >= 500 {
                        if self.potatoes >= 1000 {
                            self.potatoes -= 1000;
                        } else {
                            self.cash -= 500;
                        }
                        let spawn_pos = Self::cell_center(rand::gen_range(0, GRID), rand::gen_range(0, GRID));
                        self.ai_slaves.push(AiSlave {
                            position:      spawn_pos,
                            target_cell:   None,
                            action_timer:  0.0,
                            anim_timer:    rand::gen_range(0.0_f32, 10.0_f32),
                            facing:        rand::gen_range(0.0_f32, std::f32::consts::TAU),
                            state:         AiState::Wandering,
                            wander_target: spawn_pos,
                            wander_timer:  rand::gen_range(0.0_f32, 2.0_f32),
                            rng_offset:    rand::gen_range(0_usize, GRID * GRID),
                            wait_timer:    0.0,
                        });
                        self.set_msg("Hired AI Farm Worker Slave! They will auto-plant & harvest!");
                        self.save_game();
                    } else {
                        self.set_msg("Not enough Potatoes (1000) or Cash ($500) to hire AI Slave!");
                    }
                }
                // Key 6: Toggle AI Slave Mode (Plant & Harvest vs Plant Only)
                if is_key_pressed(KeyCode::Key6) {
                    self.ai_slave_mode = if self.ai_slave_mode == 0 { 1 } else { 0 };
                    let mode_str = if self.ai_slave_mode == 0 { "Planting & Harvesting" } else { "Planting Only" };
                    self.set_msg(&format!("AI Worker Mode set to: {}", mode_str));
                }
                // Key 7: Buy Minigun Bullets (100 Bullets for $300 Cash or 40 Potatoes)
                if is_key_pressed(KeyCode::Key7) {
                    if self.cash >= 300 || self.potatoes >= 40 {
                        if self.cash >= 300 {
                            self.cash -= 300;
                        } else {
                            self.potatoes -= 40;
                        }
                        self.bullets_count += 100;
                        self.has_unlocked_bullets = true;
                        self.set_msg("Purchased +100 Minigun Bullets from Market!");
                        self.save_game();
                    } else {
                        self.set_msg("Need $300 Cash or 40 Potatoes for 100 Minigun Bullets!");
                    }
                }
                // Key T: Buy Turret
                if is_key_pressed(KeyCode::T) {
                    self.buy_turret_upgrade();
                }
                // Key Y: Buy Iron Dome
                if is_key_pressed(KeyCode::Y) {
                    self.buy_iron_dome_upgrade();
                }
            }
        } else {
            self.market_menu_open = false;
        }

        // --- UPDATE AI FARMER SLAVES (Smart State-Machine AI) ---
        // Priority: harvest NEAREST mature crop first (to beat thieves), then plant.
        // Move speed 8.0 > thief speed 6.5 so slaves can intercept.
        // No work delay, no wander pause between jobs – slaves are always busy.
        for slave in self.ai_slaves.iter_mut() {
            slave.anim_timer += dt * 4.0;
            slave.action_timer += dt;

            // Helper closure: find the nearest harvestable cell to a position.
            // Returns (gx, gz, dist_sq). Used both in Wandering and MovingToTarget.
            // We inline it here since closures can't borrow self mutably while slave is borrowed.

            match slave.state.clone() {
                // ── WAITING FOR SEEDS ──────────────────────────────────────────────
                AiState::WaitingForSeeds => {
                    slave.wait_timer += dt;
                    // Light aimless drift so they don't freeze solid
                    slave.wander_timer -= dt;
                    if slave.wander_timer <= 0.0 {
                        let rx = slave.position.x + rand::gen_range(-5.0_f32, 5.0_f32);
                        let rz = slave.position.z + rand::gen_range(-5.0_f32, 5.0_f32);
                        slave.wander_target = vec3(
                            rx.clamp(-(GRID as f32), GRID as f32),
                            0.0,
                            rz.clamp(-(GRID as f32), GRID as f32),
                        );
                        slave.wander_timer = rand::gen_range(1.2_f32, 2.5_f32);
                    }
                    let to_w = slave.wander_target - slave.position;
                    if to_w.length() > 0.4 {
                        let dir = to_w.normalize();
                        slave.facing = dir.x.atan2(dir.z);
                        slave.position += dir * (1.5 * dt);
                    }
                    // Re-check often (every 0.2s) so slaves snap back to work quickly
                    if slave.wait_timer > 0.5 {
                        slave.wait_timer = 0.0;
                        let can_harvest = self.ai_slave_mode == 0 && self.field.iter().any(|row| {
                            row.iter().any(|c| matches!(c, CellState::Planted { growth } if *growth >= 1.0))
                        });
                        let can_plant = self.seeds > 0 && self.field.iter().any(|row| {
                            row.iter().any(|c| matches!(c, CellState::Plowed))
                        });
                        if can_harvest || can_plant {
                            slave.state = AiState::Wandering;
                        }
                    }
                }

                // ── WANDERING / TARGET SEARCH ──────────────────────────────────────
                // Slaves do a quick scan every tick. The moment they find a target they
                // commit to it immediately – no wander delay between jobs.
                AiState::Wandering => {
                    // Light drift while between jobs (keeps them spread out)
                    slave.wander_timer -= dt;
                    if slave.wander_timer <= 0.0 {
                        let rx = slave.position.x + rand::gen_range(-6.0_f32, 6.0_f32);
                        let rz = slave.position.z + rand::gen_range(-6.0_f32, 6.0_f32);
                        slave.wander_target = vec3(
                            rx.clamp(-(GRID as f32), GRID as f32),
                            0.0,
                            rz.clamp(-(GRID as f32), GRID as f32),
                        );
                        slave.wander_timer = rand::gen_range(0.5_f32, 1.5_f32);
                    }
                    let to_w = slave.wander_target - slave.position;
                    if to_w.length() > 0.4 {
                        let dir = to_w.normalize();
                        slave.facing = dir.x.atan2(dir.z);
                        slave.position += dir * (2.0 * dt);
                    }

                    // ── Smart target search: pick the NEAREST cell needing work ──
                    let pos = slave.position;
                    let mut best_harvest: Option<(usize, usize, f32)> = None; // (gx, gz, dist_sq)
                    let mut best_plant:   Option<(usize, usize, f32)> = None;

                    for gx in 0..GRID {
                        for gz in 0..GRID {
                            let cp = Self::cell_center(gx, gz);
                            let dsq = (cp - pos).length_squared();
                            match self.field[gx][gz] {
                                CellState::Planted { growth } if growth >= 1.0 && self.ai_slave_mode == 0 => {
                                    if best_harvest.map_or(true, |(_, _, d)| dsq < d) {
                                        best_harvest = Some((gx, gz, dsq));
                                    }
                                }
                                CellState::Plowed if self.seeds > 0 => {
                                    if best_plant.map_or(true, |(_, _, d)| dsq < d) {
                                        best_plant = Some((gx, gz, dsq));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    // Harvest always takes priority over planting (beats thieves)
                    let target = best_harvest
                        .map(|(gx, gz, _)| (gx, gz))
                        .or_else(|| best_plant.map(|(gx, gz, _)| (gx, gz)));

                    if let Some(t) = target {
                        slave.target_cell = Some(t);
                        slave.action_timer = 0.0;
                        slave.rng_offset = (slave.rng_offset
                            .wrapping_add(rand::gen_range(1_usize, GRID + 3)))
                            % (GRID * GRID);
                        slave.state = AiState::MovingToTarget;
                    } else if self.seeds == 0 {
                        // Nothing to harvest either – wait for seeds
                        slave.state = AiState::WaitingForSeeds;
                        slave.wait_timer = 0.0;
                    }
                }

                // ── MOVING TO TARGET ──────────────────────────────────────────────
                AiState::MovingToTarget => {
                    if let Some((gx, gz)) = slave.target_cell {
                        // Dynamic re-target: if a closer harvestable appears, switch to it.
                        // Only check for harvest (most time-sensitive) to avoid flip-flop.
                        if self.ai_slave_mode == 0 {
                            let current_dist_sq = {
                                let cp = Self::cell_center(gx, gz);
                                (cp - slave.position).length_squared()
                            };
                            let mut closer: Option<(usize, usize, f32)> = None;
                            for agx in 0..GRID {
                                for agz in 0..GRID {
                                    if agx == gx && agz == gz { continue; }
                                    if let CellState::Planted { growth } = self.field[agx][agz] {
                                        if growth >= 1.0 {
                                            let cp = Self::cell_center(agx, agz);
                                            let dsq = (cp - slave.position).length_squared();
                                            // Only re-target if at least 1.5 cells closer
                                            if dsq + (CELL * 1.5).powi(2) < current_dist_sq {
                                                if closer.map_or(true, |(_, _, d)| dsq < d) {
                                                    closer = Some((agx, agz, dsq));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if let Some((ngx, ngz, _)) = closer {
                                slave.target_cell = Some((ngx, ngz));
                            }
                        }

                        let (tgx, tgz) = slave.target_cell.unwrap();
                        let cell_pos = Self::cell_center(tgx, tgz);
                        let to_cell = cell_pos - slave.position;
                        let dist = to_cell.length();

                        if dist > 0.3 {
                            let move_dir = to_cell.normalize();
                            slave.facing = move_dir.x.atan2(move_dir.z);
                            slave.position += move_dir * (4.0 * dt);
                        } else {
                            // Snap to cell centre for cleanliness
                            slave.position = vec3(cell_pos.x, slave.position.y, cell_pos.z);
                            slave.state = AiState::Working;
                            slave.action_timer = 0.0;
                        }
                    } else {
                        slave.state = AiState::Wandering;
                    }
                }

                // ── WORKING (plant / harvest) ──────────────────────────────────────
                // No delay – act immediately on arrival.
                AiState::Working => {
                    if let Some((gx, gz)) = slave.target_cell {
                        match self.field[gx][gz] {
                            CellState::Plowed => {
                                if self.seeds > 0 {
                                    self.seeds -= 1;
                                    self.field[gx][gz] = CellState::Planted { growth: 0.0 };
                                } else {
                                    slave.state = AiState::WaitingForSeeds;
                                    slave.target_cell = None;
                                    slave.wait_timer = 0.0;
                                }
                            }
                            CellState::Planted { growth } if growth >= 1.0 && self.ai_slave_mode == 0 => {
                                self.field[gx][gz] = CellState::Plowed;
                                self.potatoes += 1;
                            }
                            _ => {} // cell changed under us (thief or player) – just move on
                        }
                        // Immediately look for next job (no wander pause)
                        if slave.state == AiState::Working {
                            slave.target_cell = None;
                            slave.state = AiState::Wandering;
                            slave.wander_timer = 0.0; // force immediate re-scan next tick
                        }
                    } else {
                        slave.state = AiState::Wandering;
                    }
                }
            }
        }


        // Smooth Camera
        let desired_target = self.farmer.position + vec3(0.0, 0.8, 0.0);
        let t = 1.0 - (-CAM_SMOOTH * dt).exp();
        self.camera.target = self.camera.target.lerp(desired_target, t);
        self.camera.position = self.camera.target + CAM_OFFSET;

        // Parallel Crop Growth using Rayon (Multi-core simulation)
        use rayon::prelude::*;
        self.field.par_iter_mut().for_each(|row| {
            for cell in row.iter_mut() {
                if let CellState::Planted { growth } = cell {
                    *growth = (*growth + dt / GROW_TIME).min(1.0);
                }
            }
        });

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

        // Choke cooldown tick
        self.thief_choke_cooldown = (self.thief_choke_cooldown - dt).max(0.0);

        // Choke/kill nearby thief ( press C / X / F when within 1.8 units )
        if (is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::F)) && self.thief_choke_cooldown <= 0.0 {
            let farmer_pos = self.farmer.position;
            if let Some(idx) = self.children.iter().position(|c| c.alive && c.position.distance(farmer_pos) < 1.8) {
                let names = ["Jabari","Kofi","Amara","Zuri","Kwame","Ayo","Chike","Nia"];
                let name = names[rand::gen_range(0, names.len())].to_string();
                self.children[idx].alive = false;
                self.thief_choke_cooldown = 60.0;
                self.choked_thief_name = name.clone();
                self.set_msg(&format!("Thief Raid: choked out {} will not spawn back for 1 minute", name));
            } else if let Some(idx) = self.children.iter().position(|c| c.alive && c.position.distance(farmer_pos) < 3.0) {
                // allow slightly further kill via message hint
                let names = ["Jabari","Kofi","Amara","Zuri","Kwame","Ayo","Chike","Nia"];
                let name = names[rand::gen_range(0, names.len())].to_string();
                self.children[idx].alive = false;
                self.thief_choke_cooldown = 60.0;
                self.choked_thief_name = name.clone();
                self.set_msg(&format!("Thief Raid: choked out {} will not spawn back for 1 minute", name));
            }
        }
        self.children.retain(|c| c.alive);

        // --- THIEF CHILDREN EVENT ---
        // Children spawn whenever turrets are unlocked, OR if player has potatoes growing/harvested
        // Blocked for 1 minute after a choke on that entity
        if self.thief_choke_cooldown > 0.0 {
            // skip spawning while that entity is choked
        } else {
        let has_planted_crops = self.field.iter().any(|row| row.iter().any(|cell| matches!(cell, CellState::Planted { .. })));
        if self.turrets_unlocked || (has_planted_crops && (self.potatoes > 0 || self.seeds > 0)) || !self.children.is_empty() {
            self.steal_timer += dt;

            // Every 6 seconds, spawn a group of thief children to raid 5 potato fields
            if self.steal_timer >= 6.0 {
                self.steal_timer = 0.0;

                // Find 5 random fully-mature potato fields to target (growth >= 1.0) — ungrown potatoes ignored
                let mut target_cells = Vec::new();
                for gx in 0..GRID {
                    for gz in 0..GRID {
                        if let CellState::Planted { growth } = self.field[gx][gz] {
                            if growth >= 1.0 {
                                target_cells.push((gx, gz));
                            }
                        }
                    }
                }

                // Spawn up to 5 thief children emerging from houses (never from markets)
                let spawn_count = 5.min(target_cells.len());
                // Filter houses that are not too close to markets (market = store, not home)
                let house_spawns: Vec<Vec3> = self.houses.iter().filter(|h| {
                    h.center.distance(WEST_MARKET_POS) > 4.5 && h.center.distance(EAST_MARKET_POS) > 4.5
                }).map(|h| h.center + vec3(0.0, 0.0, 1.8)).collect();
                for i in 0..spawn_count {
                    let (gx, gz) = target_cells[i];
                    let spawn_pos = if !house_spawns.is_empty() {
                        house_spawns[rand::gen_range(0, house_spawns.len())] + vec3(rand::gen_range(-0.5,0.5), 0.0, rand::gen_range(-0.5,0.5))
                    } else {
                        vec3(if i % 2 == 0 { -24.0 } else { 24.0 }, 0.0, if i < 2 { -24.0 } else { 24.0 })
                    };

                    self.children.push(ThiefChild {
                        position: spawn_pos,
                        target_cell: Some((gx, gz)),
                        speed: 6.5,
                        fleeing: false,
                        alive: true,
                        facing: 0.0,
                        anim_timer: rand::gen_range(0.0, 10.0),
                        harvesting_timer: 0.0,
                        hp: 3.0,
                        max_hp: 3.0,
                        has_stolen: false,
                        flee_target: spawn_pos,
                        steal_count: 0,
                    });
                }

                if spawn_count > 0 && self.msg_timer <= 0.0 {
                    self.set_msg("WARNING! Black Homeless Children raiding your Potato Fields!");
                }
            }
        }
        } // end choke block else


        // Update Thief Children AI with Harvesting & Running animations
        for child in self.children.iter_mut() {
            if !child.alive {
                continue;
            }

            child.anim_timer += dt * 10.0;

            if !child.fleeing {
                if let Some((gx, gz)) = child.target_cell {
                    // If target is no longer fully mature (harvested by player, or despawned), abandon it
                    let still_mature = matches!(self.field[gx][gz], CellState::Planted { growth } if growth >= 1.0);
                    if !still_mature {
                        child.target_cell = None;
                        child.fleeing = true;
                    } else {
                        let target_pos = Self::cell_center(gx, gz);
                        let to_target = target_pos - child.position;
                        let dist = to_target.length();

                        if dist > 0.4 {
                            let move_dir = to_target.normalize();
                            child.facing = move_dir.x.atan2(move_dir.z);
                            child.position += move_dir * (child.speed * dt);
                        } else {
                            // Steal takes longer: 2.5s per swipe, 3 swipes on same mature potato before it disappears
                            child.harvesting_timer += dt;
                            if child.harvesting_timer >= 2.5 {
                                child.harvesting_timer = 0.0;
                                // Re-check maturity at moment of swipe - ungrown potatoes are ignored
                                let still_mature2 = matches!(self.field[gx][gz], CellState::Planted { growth } if growth >= 1.0);
                                if !still_mature2 {
                                    child.target_cell = None;
                                    child.fleeing = true;
                                } else {
                                    child.steal_count += 1;
                                    if child.steal_count >= 3 {
                                        self.field[gx][gz] = CellState::Plowed;
                                        child.has_stolen = true;
                                        child.fleeing = true;
                                        child.target_cell = None;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    child.fleeing = true;
                }
            } else {
                // Flee back into the house they came from
                let to_home = child.flee_target - child.position;
                let dist = to_home.length();
                if dist > 0.3 {
                    let dir = to_home.normalize();
                    child.facing = dir.x.atan2(dir.z);
                    child.position += dir * (child.speed * dt);
                } else {
                    child.position = child.flee_target;
                }
            }
        }

        // Thieves entering house despawn; inventory potatoes no longer deducted (plot loss is the penalty)
        for child in self.children.iter_mut() {
            if child.alive && child.fleeing && child.position.distance(child.flee_target) < 0.5 {
                child.alive = false;
            }
        }

        // Remove dead children (escaped into house already marked not alive)
        self.children.retain(|c| c.alive);

        // --- AFRICAN REBEL GUNBOATS RIVER RAID EVENT ---
        // Spawn African Rebel gunboat raids twice as often as B-2 bomber (every 30s) if player has deployed turrets!
        if !self.turrets.is_empty() {
            self.raid_timer += dt;
            if self.raid_timer >= 30.0 {
                self.raid_timer = 0.0;

                // Only spawn a gunboat if current rebels count is < 3
                if self.rebels.len() < 3 {
                let spawn_z = if rand::gen_range(0.0, 1.0) < 0.5 { -36.0 } else { 36.0 };
                let target_z = 0.0; // Dock near bridge

                self.gunboats.push(GunBoat {
                    position: vec3(-31.0, -0.1, spawn_z),
                    target_z,
                    hp: 20.0,
                    max_hp: 20.0,
                    disembarked: false,
                    disembark_timer: 0.0,
                    alive: true,
                });

                self.set_msg("NAVY ALERT! African Rebels GunBoats entering The River!");
            }
        }
    }

        // Update Gunboats
        for boat in self.gunboats.iter_mut() {
            if !boat.alive {
                continue;
            }

            let dist_to_dock = (boat.target_z - boat.position.z).abs();
            if dist_to_dock > 0.5 && !boat.disembarked {
                let dir_z = if boat.target_z > boat.position.z { 1.0 } else { -1.0 };
                boat.position.z += dir_z * 4.5 * dt;
            } else {
                // Docked at bridge - timer to disembark rebels if turrets don't destroy it!
                boat.disembark_timer += dt;
                if boat.disembark_timer >= 4.0 && !boat.disembarked {
                    boat.disembarked = true;
                    // Disembark up to 3 armed rebels to raid farm (max 3 allowed overall)
                    let spawn_count = 3_usize.saturating_sub(self.rebels.len());
                    for i in 0..spawn_count {
                        let offset_z = -1.0 + i as f32 * 1.0;
                        self.rebels.push(Rebel {
                            position: vec3(-27.0, 0.0, offset_z),
                            target_cell: Some((rand::gen_range(0, GRID), rand::gen_range(0, GRID))),
                            speed: 5.2,
                            hp: 5.0,
                            max_hp: 5.0,
                            alive: true,
                            facing: 1.57,
                            anim_timer: 0.0,
                            raiding_timer: 0.0,
                            shoot_cooldown: rand::gen_range(0.2, 1.0),
                        });
                    }
                }
            }
        }
        self.gunboats.retain(|b| b.alive);

        // Update Disembarked Rebels & AK-47 Shooting at Farmer
        let farmer_pos = self.farmer.position;
        for rebel in self.rebels.iter_mut() {
            if !rebel.alive {
                continue;
            }

            rebel.anim_timer += dt * 8.0;
            rebel.shoot_cooldown = (rebel.shoot_cooldown - dt).max(0.0);

            // Turn to face farmer and fire AK-47 if within range (25 units)
            let to_farmer = farmer_pos - rebel.position;
            let dist_to_farmer = to_farmer.length();
            if dist_to_farmer < 25.0 {
                rebel.facing = to_farmer.x.atan2(to_farmer.z);
                if rebel.shoot_cooldown <= 0.0 {
                    rebel.shoot_cooldown = 0.8;
                    let muzzle = rebel.position + vec3(0.0, 1.0, 0.0) + vec3(rebel.facing.sin(), 0.0, rebel.facing.cos()) * 0.5;
                    let dir = (farmer_pos + vec3(0.0, 0.8, 0.0) - muzzle).normalize();
                    self.rebel_bullets.push(RebelBullet {
                        position: muzzle,
                        velocity: dir * 35.0,
                        life: 1.2,
                    });
                }
            } else if let Some((gx, gz)) = rebel.target_cell {
                let target_pos = Self::cell_center(gx, gz);
                let to_target = target_pos - rebel.position;
                let dist = to_target.length();

                if dist > 0.4 {
                    let move_dir = to_target.normalize();
                    rebel.facing = move_dir.x.atan2(move_dir.z);
                    rebel.position += move_dir * (rebel.speed * dt);
                } else {
                    rebel.raiding_timer += dt;
                    if rebel.raiding_timer >= 1.5 {
                        rebel.raiding_timer = 0.0;
                        rebel.target_cell = Some((rand::gen_range(0, GRID), rand::gen_range(0, GRID)));
                    }
                }
            }
        }
        self.rebels.retain(|r| r.alive);

        // Update Rebel AK-47 Bullets (Damages Farmer only)
        for bullet in self.rebel_bullets.iter_mut() {
            bullet.position += bullet.velocity * dt;
            bullet.life -= dt;

            // Damage Farmer on hit (12 damage per AK-47 bullet)
            if bullet.position.distance(self.farmer.position + vec3(0.0, 0.8, 0.0)) < 1.0 {
                self.farmer.hp = (self.farmer.hp - 12.0).max(0.0);
                bullet.life = 0.0;
                if self.farmer.hp <= 0.0 {
                    self.game_over = true;
                    // Delete save file on permanent death!
                    let _ = std::fs::remove_file(SAVE_FILE);
                }
            }
        }
        self.rebel_bullets.retain(|b| b.life > 0.0);

        // --- AUTOMATED DEFENSE TURRETS ENGINE ---
        if !self.turrets.is_empty() {
            for turret in self.turrets.iter_mut() {
                turret.fire_cooldown = (turret.fire_cooldown - dt).max(0.0);

                if turret.fire_cooldown <= 0.0 {
                    let t_pos = turret.position + vec3(0.0, 1.2, 0.0);

                    // Target priority: Gunboats > Rebels > Thief Children
                    let mut target_found = None;

                    for boat in self.gunboats.iter() {
                        if boat.alive && t_pos.distance(boat.position) < 26.0 {
                            target_found = Some((boat.position + vec3(0.0, 0.8, 0.0), 0));
                            break;
                        }
                    }

                    if target_found.is_none() {
                        for rebel in self.rebels.iter() {
                            if rebel.alive && t_pos.distance(rebel.position) < 22.0 {
                                target_found = Some((rebel.position + vec3(0.0, 0.6, 0.0), 1));
                                break;
                            }
                        }
                    }

                    if target_found.is_none() {
                        for child in self.children.iter() {
                            if child.alive && t_pos.distance(child.position) < 18.0 {
                                target_found = Some((child.position + vec3(0.0, 0.6, 0.0), 2));
                                break;
                            }
                        }
                    }

                    if let Some((target_pos, _type_id)) = target_found {
                        let dir = (target_pos - t_pos).normalize();

                        self.turret_bullets.push(BulletParticle {
                            position: t_pos,
                            velocity: dir * 48.0,
                            life: 0.6,
                        });

                        turret.fire_cooldown = 0.22;
                    }
                }
            }

            // Update Turret Bullets & Collision with Targets
            for bullet in self.turret_bullets.iter_mut() {
                bullet.position += bullet.velocity * dt;
                bullet.life -= dt;

                // Check collision with Gunboats
                for boat in self.gunboats.iter_mut() {
                    if boat.alive && bullet.position.distance(boat.position + vec3(0.0, 0.8, 0.0)) < 2.2 {
                        boat.hp -= 1.0;
                        if boat.hp <= 0.0 {
                            boat.alive = false;
                        }
                        bullet.life = 0.0;
                        break;
                    }
                }

                // Check collision with Rebels
                if bullet.life > 0.0 {
                    for rebel in self.rebels.iter_mut() {
                        if rebel.alive && bullet.position.distance(rebel.position + vec3(0.0, 0.6, 0.0)) < 0.9 {
                            rebel.hp -= 1.0;
                            if rebel.hp <= 0.0 {
                                rebel.alive = false;
                            }
                            bullet.life = 0.0;
                            break;
                        }
                    }
                }

                // Check bullet collision with thief children
                if bullet.life > 0.0 {
                    for child in self.children.iter_mut() {
                        if child.alive && bullet.position.distance(child.position + vec3(0.0, 0.6, 0.0)) < 0.8 {
                            child.hp -= 1.0;
                            if child.hp <= 0.0 {
                                child.alive = false;
                            }
                            bullet.life = 0.0;
                            break;
                        }
                    }
                }
            }
            self.turret_bullets.retain(|b| b.life > 0.0);
        }

        // Automated & Manual Heavy Minigun Firing (Auto-targets Rebels, Gunboats & Thief Children!)
        self.minigun_cooldown = (self.minigun_cooldown - dt).max(0.0);
        if self.minigun_unlocked && self.bullets_count > 0 && self.minigun_cooldown <= 0.0 {
            let f_pos = self.farmer.position + vec3(0.0, 0.8, 0.0);

            // Auto-Target Priority: Rebels > Gunboats > Thief Children
            let mut target_found = None;
            for rebel in self.rebels.iter() {
                if rebel.alive && f_pos.distance(rebel.position) < 28.0 {
                    target_found = Some(rebel.position + vec3(0.0, 0.6, 0.0));
                    break;
                }
            }
            if target_found.is_none() {
                for boat in self.gunboats.iter() {
                    if boat.alive && f_pos.distance(boat.position) < 32.0 {
                        target_found = Some(boat.position + vec3(0.0, 0.8, 0.0));
                        break;
                    }
                }
            }
            if target_found.is_none() {
                for child in self.children.iter() {
                    if child.alive && f_pos.distance(child.position) < 25.0 {
                        target_found = Some(child.position + vec3(0.0, 0.6, 0.0));
                        break;
                    }
                }
            }

            let manual_fire = is_mouse_button_down(MouseButton::Left) || is_key_down(KeyCode::F) || is_key_down(KeyCode::M);

            if let Some(target_pos) = target_found {
                // Auto-aim towards nearest threat
                let dir = (target_pos - f_pos).normalize();
                self.farmer.facing = dir.x.atan2(dir.z);

                self.bullets_count -= 1;
                self.minigun_cooldown = 0.07; // Rapid fire auto-turret minigun!
                let muzzle = self.farmer.position + vec3(0.0, 0.9, 0.0) + dir * 0.6;
                self.minigun_bullets.push(MinigunBullet {
                    position: muzzle,
                    velocity: dir * 60.0,
                    life: 1.0,
                });
            } else if manual_fire {
                self.bullets_count -= 1;
                self.minigun_cooldown = 0.07;
                let dir = vec3(self.farmer.facing.sin(), 0.0, self.farmer.facing.cos()).normalize();
                let muzzle = self.farmer.position + vec3(0.0, 0.9, 0.0) + dir * 0.6;
                self.minigun_bullets.push(MinigunBullet {
                    position: muzzle,
                    velocity: dir * 60.0,
                    life: 1.0,
                });
            }
        }

        // Update Minigun Bullets & Damage Handling
        for bullet in self.minigun_bullets.iter_mut() {
            bullet.position += bullet.velocity * dt;
            bullet.life -= dt;

            // Damage Rebels
            for rebel in self.rebels.iter_mut() {
                if rebel.alive && bullet.position.distance(rebel.position + vec3(0.0, 0.6, 0.0)) < 1.2 {
                    rebel.hp -= 3.0;
                    if rebel.hp <= 0.0 {
                        rebel.alive = false;
                    }
                    bullet.life = 0.0;
                    break;
                }
            }

            // Damage Gunboats
            if bullet.life > 0.0 {
                for boat in self.gunboats.iter_mut() {
                    if boat.alive && bullet.position.distance(boat.position + vec3(0.0, 0.8, 0.0)) < 2.5 {
                        boat.hp -= 2.5;
                        if boat.hp <= 0.0 {
                            boat.alive = false;
                        }
                        bullet.life = 0.0;
                        break;
                    }
                }
            }

            // Damage Thief Children
            if bullet.life > 0.0 {
                for child in self.children.iter_mut() {
                    if child.alive && bullet.position.distance(child.position + vec3(0.0, 0.6, 0.0)) < 1.0 {
                        child.hp -= 2.0;
                        if child.hp <= 0.0 {
                            child.alive = false;
                        }
                        bullet.life = 0.0;
                        break;
                    }
                }
            }
        }
        self.minigun_bullets.retain(|b| b.life > 0.0);

        // Ground Loot Pickup (Player walks over dropped loot)
        let f_pos = self.farmer.position;
        let mut picked_msg = None;
        for loot in self.dropped_loot.iter_mut() {
            if loot.amount > 0 && f_pos.distance(loot.position) < 1.6 {
                match loot.loot_type {
                    LootType::BloodDiamonds => {
                        let already_had = self.has_unlocked_blood_diamonds;
                        self.blood_diamonds += loot.amount;
                        self.has_unlocked_blood_diamonds = true;
                        if already_had {
                            picked_msg = Some(format!("Picked up {} Blood Diamonds! (Converted to currency, already unlocked!)", loot.amount));
                        } else {
                            picked_msg = Some(format!("UNLOCKED NEW CURRENCY: {} Blood Diamonds!", loot.amount));
                        }
                    }
                    LootType::Cash => {
                        let already_had = self.has_unlocked_cash;
                        self.cash += loot.amount * 500;
                        self.has_unlocked_cash = true;
                        if already_had {
                            picked_msg = Some(format!("Picked up ${} Cash! (Converted to currency, already unlocked!)", loot.amount * 500));
                        } else {
                            picked_msg = Some(format!("UNLOCKED NEW CURRENCY: ${} Cash Money!", loot.amount * 500));
                        }
                    }
                    LootType::PantherStatue => {
                        let already_had = self.has_unlocked_panther_statue;
                        self.panther_statues += loot.amount;
                        self.has_unlocked_panther_statue = true;
                        if already_had {
                            picked_msg = Some(format!("Picked up {} Panther Statue! (Converted to currency, already unlocked!)", loot.amount));
                        } else {
                            picked_msg = Some(format!("UNLOCKED NEW CURRENCY: {} Panther Statue!", loot.amount));
                        }
                    }
                    LootType::Gold => {
                        let already_had = self.has_unlocked_gold;
                        self.gold += loot.amount * 10;
                        self.has_unlocked_gold = true;
                        if already_had {
                            picked_msg = Some(format!("Picked up {} Gold Bars! (Converted to currency, already unlocked!)", loot.amount * 10));
                        } else {
                            picked_msg = Some(format!("UNLOCKED NEW CURRENCY: {} Gold Bars!", loot.amount * 10));
                        }
                    }
                    LootType::Bullets => {
                        let already_had = self.has_unlocked_bullets;
                        self.bullets_count += loot.amount * 50;
                        self.has_unlocked_bullets = true;
                        if already_had {
                            picked_msg = Some(format!("Picked up {} Bullets! (Converted to currency/ammo, already unlocked!)", loot.amount * 50));
                        } else {
                            picked_msg = Some(format!("UNLOCKED NEW ITEM: {} Minigun Bullets!", loot.amount * 50));
                        }
                    }
                    LootType::Minigun => {
                        let already_had = self.has_unlocked_minigun;
                        self.minigun_unlocked = true;
                        self.has_unlocked_minigun = true;
                        self.bullets_count += 200;
                        self.has_unlocked_bullets = true;
                        if already_had {
                            picked_msg = Some("Picked up duplicate Minigun! (Converted to currency: +200 Minigun Bullets!)".to_string());
                        } else {
                            picked_msg = Some("UNLOCKED THE HEAVY MINIGUN! Press [F] or LMB to Shoot!".to_string());
                        }
                    }
                }
                loot.amount = 0;
            }
        }
        if let Some(msg) = picked_msg {
            self.set_msg(&msg);
            self.save_game();
        }
        self.dropped_loot.retain(|l| l.amount > 0);

        // Update Crashing B2 Stealth Bombers (Falling, rotating, exploding on ground impact)
        let mut crashed_impacts = Vec::new();
        for bomber in self.crashing_bombers.iter_mut() {
            bomber.velocity.y -= 18.0 * dt; // Gravity pull
            bomber.position += bomber.velocity * dt;
            bomber.rotation += bomber.rot_speed * dt;
            bomber.life -= dt;

            // Check ground impact (y <= 0.2)
            if bomber.position.y <= 0.2 && bomber.life > 0.0 {
                bomber.life = 0.0;
                crashed_impacts.push(vec3(bomber.position.x, 0.1, bomber.position.z));
            }
        }
        self.crashing_bombers.retain(|b| b.life > 0.0);

        for impact_pos in crashed_impacts {
            self.spawn_sparkles(impact_pos);

            // Select 1 single random loot item per B-2 Bomber shot down
            let items = [
                LootType::BloodDiamonds,
                LootType::Cash,
                LootType::PantherStatue,
                LootType::Gold,
                LootType::Bullets,
                LootType::Minigun,
            ];
            let selected_loot = items[rand::gen_range(0, items.len())];

            self.dropped_loot.push(DroppedLoot {
                loot_type: selected_loot,
                position: impact_pos,
                amount: 1,
            });
            self.set_msg("B2 BOMBER CRASHED! Secret Loot dropped on farm ground!");
        }

        // --- ISRAELI IRON DOME BATTERY SYSTEM ---
        for dome in self.iron_domes.iter_mut() {
            dome.cooldown = (dome.cooldown - dt).max(0.0);

            if dome.cooldown <= 0.0 && self.air_event.active {
                let target = self.air_event.bomber_pos;
                let dome_pos = dome.position + vec3(0.0, 1.5, 0.0);

                // Only shoot down B-2 Bomber when it is strictly over/after the river and on the farm field (-26.0 <= target.x <= 15.0)
                if target.x >= -26.0 && target.x <= 15.0 && dome_pos.distance(target) < 140.0 {
                    self.iron_dome_missiles.push(IronDomeMissile {
                        position: dome_pos,
                        target_pos: target,
                        speed: 75.0,
                        life: 2.5,
                    });
                    dome.cooldown = 1.2;
                }
            }
        }

        // Update Iron Dome Missiles & Intercepting B2 Bomber
        let mut intercept_data = None;
        for missile in self.iron_dome_missiles.iter_mut() {
            let dir = (missile.target_pos - missile.position).normalize();
            missile.position += dir * (missile.speed * dt);
            missile.life -= dt;

            // Intercept check against active B2 bomber
            if self.air_event.active && missile.position.distance(self.air_event.bomber_pos) < 5.0 {
                intercept_data = Some(self.air_event.bomber_pos);
                missile.life = 0.0;
            }
        }
        if let Some(pos) = intercept_data {
            self.air_event.active = false; // Intercepted!
            self.spawn_sparkles(pos);
            
            // Clamp crash landing target safely within open farm field bounds (avoiding river & surrounding houses)
            let safe_x = pos.x.clamp(-20.0, 18.0);
            let safe_z = pos.z.clamp(-15.0, 15.0);

            // Trigger B2 Bomber crash trajectory landing safely on open farm ground!
            self.crashing_bombers.push(CrashingBomber {
                position: vec3(safe_x, pos.y, safe_z),
                velocity: vec3(0.0, -14.0, 0.0),
                rotation: 0.0,
                rot_speed: rand::gen_range(3.0, 8.0),
                life: 2.0,
            });

            self.set_msg("IRON DOME SHOT DOWN B-2 BOMBER! Crashing onto the ground!");
        }
        self.iron_dome_missiles.retain(|m| m.life > 0.0);

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
