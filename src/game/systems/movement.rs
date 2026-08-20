use macroquad::prelude::*;
use crate::constants::*;
use super::super::Game;

impl Game {
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

        // 3. Placed Turrets Physical Solid Collision Box (cannot walk through)
        for t in &self.turrets {
            if target_pos.distance(t.position) < CELL * 0.5 {
                return true;
            }
        }

        // 4. Placed Iron Domes Physical Solid Collision Box (cannot walk through)
        for d in &self.iron_domes {
            if target_pos.distance(d.position) < CELL * 0.5 {
                return true;
            }
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
}
