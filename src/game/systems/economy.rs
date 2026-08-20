use macroquad::prelude::*;
use crate::constants::*;
use crate::types::*;
use super::super::Game;

impl Game {
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

    pub fn get_placement_cell(&self) -> Option<(usize, usize)> {
        let f = &self.farmer;
        let dx = f.facing.sin().round() as i32;
        let dz = f.facing.cos().round() as i32;

        let front_gx = f.grid_x + dx;
        let front_gz = f.grid_z + dz;

        // 1. Prefer cell directly in front if inside field and plowed
        if front_gx >= 0 && front_gx < GRID as i32 && front_gz >= 0 && front_gz < GRID as i32 {
            let fgx = front_gx as usize;
            let fgz = front_gz as usize;
            if self.field[fgx][fgz] == CellState::Plowed && !self.is_occupied_by_structure(fgx, fgz) {
                return Some((fgx, fgz));
            }
        }

        // 2. Fallback to cell under farmer if plowed
        if f.grid_x >= 0 && f.grid_x < GRID as i32 && f.grid_z >= 0 && f.grid_z < GRID as i32 {
            let cgx = f.grid_x as usize;
            let cgz = f.grid_z as usize;
            if self.field[cgx][cgz] == CellState::Plowed && !self.is_occupied_by_structure(cgx, cgz) {
                return Some((cgx, cgz));
            }
        }

        None
    }

    pub fn place_iron_dome(&mut self) -> bool {
        if self.iron_domes_in_inventory == 0 {
            self.set_msg("No Iron Domes in inventory! Buy them at Market for 120 Potatoes.");
            return false;
        }

        let Some((gx, gz)) = self.get_placement_cell() else {
            self.set_msg("Iron Dome can only be placed on plowed soil! Plow a cell first (hold Space).");
            return false;
        };

        if self.is_occupied_by_structure(gx, gz) {
            self.set_msg("Cell already occupied by a structure!");
            return false;
        }

        let snapped = Self::cell_center(gx, gz);

        if self.hits_solid_obstacle(snapped) {
            self.set_msg("Cannot place Iron Dome inside an obstacle!");
            return false;
        }

        self.field[gx][gz] = CellState::Grass;
        self.iron_domes.push(IronDome {
            position: snapped,
            cooldown: 0.0,
            angle: 0.0,
        });
        self.iron_domes_in_inventory -= 1;
        self.spawn_sparkles(snapped + vec3(0.0, 1.0, 0.0));
        self.set_msg(&format!("Iron Dome deployed! Auto-intercepting jets with missiles! (In hand: {})", self.iron_domes_in_inventory));
        true
    }

    pub fn pickup_iron_dome(&mut self) -> bool {
        let pos = self.farmer.position;
        if let Some(idx) = self.iron_domes.iter().position(|d| d.position.distance(pos) < 2.8) {
            self.iron_domes.remove(idx);
            self.iron_domes_in_inventory += 1;
            self.set_msg(&format!("Picked up Iron Dome! Inventory: {}", self.iron_domes_in_inventory));
            return true;
        }
        false
    }

    pub fn pickup_turret(&mut self) -> bool {
        let pos = self.farmer.position;
        if let Some(idx) = self.turrets.iter().position(|t| t.position.distance(pos) < 2.8) {
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

        let Some((gx, gz)) = self.get_placement_cell() else {
            self.set_msg("Turret can only be placed on plowed soil! Plow a cell first (hold Space).");
            return false;
        };

        if self.is_occupied_by_structure(gx, gz) {
            self.set_msg("Cell already occupied by a structure!");
            return false;
        }

        let snapped = Self::cell_center(gx, gz);

        if self.hits_solid_obstacle(snapped) {
            self.set_msg("Cannot place turret inside an obstacle!");
            return false;
        }

        self.field[gx][gz] = CellState::Grass;
        self.turrets.push(Turret {
            position: snapped,
            fire_cooldown: 0.0,
            angle: 0.0,
        });
        self.turrets_in_inventory -= 1;
        self.spawn_sparkles(snapped + vec3(0.0, 1.0, 0.0));
        self.set_msg(&format!("Turret placed down! (Remaining in inventory: {})", self.turrets_in_inventory));
        true
    }
}
