use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub const GRID: usize = 20;
pub const CELL: f32 = 2.0;
pub const FIELD_HALF: f32 = GRID as f32 * CELL / 2.0; // 20.0

pub const GROW_TIME: f32 = 18.0;

// Movement speed in units per second (grid step transition speed)
pub const MOVE_SPEED: f32 = 12.0;
pub const CAM_SMOOTH: f32 = 10.0;

// Camera offset
pub const CAM_OFFSET: Vec3 = vec3(24.0, 30.0, 24.0);

pub const STEP_REPEAT: f32 = 0.12;

// Map Boundaries (Invisible Wall Limit for Grid coordinates)
pub const MAP_LIMIT_X_MIN: f32 = -52.0;
pub const MAP_LIMIT_X_MAX: f32 = 52.0;
pub const MAP_LIMIT_Z_MIN: f32 = -34.0;
pub const MAP_LIMIT_Z_MAX: f32 = 34.0;

// River boundaries (River runs North-South at x around -35.0 to -27.0)
pub const RIVER_X_MIN: f32 = -35.0;
pub const RIVER_X_MAX: f32 = -27.0;

// Wooden Shack Bridge bounds across river
pub const BRIDGE_Z_CENTER: f32 = 0.0;
pub const BRIDGE_Z_HALF_WIDTH: f32 = 2.2;

// Opposite Market locations on West and East sides of the field
pub const WEST_MARKET_POS: Vec3 = vec3(-FIELD_HALF - 1.2, 0.0, 0.0);
pub const EAST_MARKET_POS: Vec3 = vec3(FIELD_HALF + 1.2, 0.0, 0.0);

pub const POTATO_TO_SEED: u32 = 4;
pub const TURRET_COST: u32 = 50;
pub const SAVE_FILE: &str = "savegame.json";

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Grass,
    Plowed,
    Planted { growth: f32 },
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum CellStateSave {
    Grass,
    Plowed,
    Planted { growth: f32 },
}

impl From<CellState> for CellStateSave {
    fn from(state: CellState) -> Self {
        match state {
            CellState::Grass => CellStateSave::Grass,
            CellState::Plowed => CellStateSave::Plowed,
            CellState::Planted { growth } => CellStateSave::Planted { growth },
        }
    }
}

impl From<CellStateSave> for CellState {
    fn from(save: CellStateSave) -> Self {
        match save {
            CellStateSave::Grass => CellState::Grass,
            CellStateSave::Plowed => CellState::Plowed,
            CellStateSave::Planted { growth } => CellState::Planted { growth },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum LootType {
    BloodDiamonds,
    Cash,
    PantherStatue,
    Gold,
    Bullets,
    Minigun,
}

pub struct DroppedLoot {
    pub loot_type: LootType,
    pub position: Vec3,
    pub amount: u32,
}

pub struct CrashingBomber {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: f32,
    pub rot_speed: f32,
    pub life: f32,
}

pub struct MinigunBullet {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub seeds: u32,
    pub potatoes: u32,
    pub farmer_grid_x: i32,
    pub farmer_grid_z: i32,
    pub field: Vec<Vec<CellStateSave>>,
    pub turrets_unlocked: bool,
    #[serde(default)]
    pub turrets_in_inventory: u32,
    #[serde(default)]
    pub turret_positions: Vec<(f32, f32, f32)>,
    #[serde(default)]
    pub iron_dome_positions: Vec<(f32, f32, f32)>,
    #[serde(default)]
    pub iron_domes_in_inventory: u32,
    #[serde(default)]
    pub blood_diamonds: u32,
    #[serde(default)]
    pub cash: u32,
    #[serde(default)]
    pub panther_statues: u32,
    #[serde(default)]
    pub gold: u32,
    #[serde(default)]
    pub bullets_count: u32,
    #[serde(default)]
    pub minigun_unlocked: bool,
    #[serde(default)]
    pub has_unlocked_blood_diamonds: bool,
    #[serde(default)]
    pub has_unlocked_cash: bool,
    #[serde(default)]
    pub has_unlocked_panther_statue: bool,
    #[serde(default)]
    pub has_unlocked_gold: bool,
    #[serde(default)]
    pub has_unlocked_bullets: bool,
    #[serde(default)]
    pub has_unlocked_minigun: bool,
    #[serde(default)]
    pub ai_slaves_count: u32,
    #[serde(default)]
    pub ai_slave_mode: u8, // 0 = Plant & Harvest, 1 = Plant Only
}

pub struct DirtParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
    pub color: Color,
}

pub struct SparkleParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
}

pub const IRON_DOME_COST: u32 = 120;

use macroquad::audio::Sound;

pub struct SoundEffects {
    pub turret_fire: Option<Sound>,
    pub jet_flyby: Option<Sound>,
    pub jet_shoot: Option<Sound>,
    pub iron_dome_intercept: Option<Sound>,
    pub boat_engine: Option<Sound>,
    pub thief_giggle: Option<Sound>,
}

pub struct BulletParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
}

pub struct IronDomeMissile {
    pub position: Vec3,
    pub target_pos: Vec3,
    pub speed: f32,
    pub life: f32,
}

pub struct IronDome {
    pub position: Vec3,
    pub cooldown: f32,
}

pub struct GunBoat {
    pub position: Vec3,
    pub target_z: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub disembarked: bool,
    pub disembark_timer: f32,
    pub alive: bool,
}

pub struct RebelBullet {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
}

pub struct Rebel {
    pub position: Vec3,
    pub target_cell: Option<(usize, usize)>,
    pub speed: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub alive: bool,
    pub facing: f32,
    pub anim_timer: f32,
    pub raiding_timer: f32,
    pub shoot_cooldown: f32,
}

pub struct AirEvent {
    pub active: bool,
    pub timer: f32,
    pub fly_time: f32,
    pub bomber_pos: Vec3,
    pub jet1_pos: Vec3,
    pub jet2_pos: Vec3,
    pub bullets: Vec<BulletParticle>,
}

pub struct ThiefChild {
    pub position: Vec3,
    pub target_cell: Option<(usize, usize)>,
    pub speed: f32,
    pub fleeing: bool,
    pub alive: bool,
    pub facing: f32,
    pub anim_timer: f32,
    pub harvesting_timer: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub has_stolen: bool,
}

pub struct Turret {
    pub position: Vec3,
    pub fire_cooldown: f32,
}

pub struct AiSlave {
    pub position: Vec3,
    pub target_cell: Option<(usize, usize)>,
    pub action_timer: f32,
    pub anim_timer: f32,
    pub facing: f32,
}

pub struct Farmer {
    pub grid_x: i32,
    pub grid_z: i32,
    pub position: Vec3,
    pub facing: f32,
    pub plowing: bool,
    pub step_cooldown: f32,
    pub hp: f32,
    pub max_hp: f32,
}

pub struct CameraState {
    pub position: Vec3,
    pub target: Vec3,
}

impl CameraState {
    #[inline(always)]
    pub fn is_in_view(&self, point: Vec3, radius: f32) -> bool {
        // Orthographics camera frustum check centered around target position
        let min_x = self.target.x - 30.0 - radius;
        let max_x = self.target.x + 30.0 + radius;
        let min_z = self.target.z - 25.0 - radius;
        let max_z = self.target.z + 25.0 + radius;
        point.x >= min_x && point.x <= max_x && point.z >= min_z && point.z <= max_z
    }
}

#[derive(Clone, Copy)]
pub struct HouseBounds {
    pub center: Vec3,
    pub min_x: f32,
    pub max_x: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub style: usize,
}

// Deterministic pseudo-random float [0..1] based on grid coordinates and seed index
#[inline(always)]
pub fn cell_hash(gx: usize, gz: usize, index: u32) -> f32 {
    let mut h = (gx as u32).wrapping_mul(374761393)
        ^ (gz as u32).wrapping_mul(668265263)
        ^ index.wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    (h as f32) / (u32::MAX as f32)
}
