use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub const GRID: usize = 20;
pub const CELL: f32 = 2.0;
pub const FIELD_HALF: f32 = GRID as f32 * CELL / 2.0; // 20.0

pub const GROW_TIME: f32 = 18.0;

// Movement speed in units per second
pub const MOVE_SPEED: f32 = 10.0;
pub const CAM_SMOOTH: f32 = 8.0;

// Wider camera to view the full field, village, river, and markets
pub const CAM_OFFSET: Vec3 = vec3(24.0, 30.0, 24.0);

pub const STEP_REPEAT: f32 = 0.12;

// Map Boundaries (Invisible Wall Limit)
pub const MAP_LIMIT_X_MIN: f32 = -52.0;
pub const MAP_LIMIT_X_MAX: f32 = 52.0;
pub const MAP_LIMIT_Z_MIN: f32 = -34.0;
pub const MAP_LIMIT_Z_MAX: f32 = 34.0;

// River boundaries (River runs North-South at x around -28.0 to -34.0)
pub const RIVER_X_MIN: f32 = -35.0;
pub const RIVER_X_MAX: f32 = -27.0;

// Wooden Shack Bridge bounds across river
pub const BRIDGE_Z_CENTER: f32 = 0.0;
pub const BRIDGE_Z_HALF_WIDTH: f32 = 2.2;

// Opposite Market locations on West and East sides of the field
pub const WEST_MARKET_POS: Vec3 = vec3(-FIELD_HALF - 1.2, 0.0, 0.0);
pub const EAST_MARKET_POS: Vec3 = vec3(FIELD_HALF + 1.2, 0.0, 0.0);

pub const POTATO_TO_SEED: u32 = 4;
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

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub seeds: u32,
    pub potatoes: u32,
    pub farmer_grid_x: usize,
    pub farmer_grid_z: usize,
    pub field: Vec<Vec<CellStateSave>>,
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

pub struct BulletParticle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub life: f32,
}

pub struct AirEvent {
    pub active: bool,
    pub timer: f32,       // Counts up to 60.0s cycle
    pub fly_time: f32,    // Progress of flyby (0.0 to 1.0)
    pub bomber_pos: Vec3,
    pub jet1_pos: Vec3,
    pub jet2_pos: Vec3,
    pub bullets: Vec<BulletParticle>,
}

pub struct Farmer {
    pub grid_x: usize,
    pub grid_z: usize,
    pub position: Vec3,
    pub facing: f32,
    pub plowing: bool,
    pub step_cooldown: f32,
}

pub struct CameraState {
    pub position: Vec3,
    pub target: Vec3,
}

// Deterministic pseudo-random float [0..1] based on grid coordinates and seed index
pub fn cell_hash(gx: usize, gz: usize, index: u32) -> f32 {
    let mut h = (gx as u32).wrapping_mul(374761393)
        ^ (gz as u32).wrapping_mul(668265263)
        ^ index.wrapping_mul(2246822519);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    (h as f32) / (u32::MAX as f32)
}
