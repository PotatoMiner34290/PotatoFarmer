use macroquad::prelude::*;
use crate::constants::*;
use crate::types::*;
use super::super::Game;

pub fn grid_to_world(gx: i32, gz: i32) -> Vec3 {
    vec3(-FIELD_HALF + gx as f32 * CELL + CELL / 2.0, 0.0, -FIELD_HALF + gz as f32 * CELL + CELL / 2.0)
}

impl Game {
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
}
