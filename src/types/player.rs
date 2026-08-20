use macroquad::prelude::Vec3;

pub struct Farmer {
    pub grid_x: i32,
    pub grid_z: i32,
    pub position: Vec3,
    pub facing: f32,
    pub plowing: bool,
    pub step_cooldown: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub step_sound_timer: f32,
}

/// Behaviour state for an AI slave
#[derive(Clone, PartialEq)]
pub enum AiState {
    /// Wandering randomly – no work to do or seeds ran out
    Wandering,
    /// Has a target cell and is walking toward it
    MovingToTarget,
    /// Arrived at cell, performing the action
    Working,
    /// No seeds and no harvestable crops – standing idle briefly
    WaitingForSeeds,
}

pub struct AiSlave {
    pub position:      Vec3,
    pub target_cell:   Option<(usize, usize)>,
    pub action_timer:  f32,
    pub anim_timer:    f32,
    pub facing:        f32,
    /// Current behaviour state
    pub state:         AiState,
    /// Wander destination (world-space)
    pub wander_target: Vec3,
    /// How long until we pick a new wander destination
    pub wander_timer:  f32,
    /// Per-slave random seed offset used to stagger searches
    pub rng_offset:    usize,
    /// Accumulates time the slave has been waiting for seeds
    pub wait_timer:    f32,
    /// Timer for footstep sound cadence
    pub step_timer:    f32,
    /// Timer until next voice sound/chatter
    pub talk_timer:    f32,
    /// Search cooldown to optimize target scan performance
    pub search_cooldown: f32,
}

