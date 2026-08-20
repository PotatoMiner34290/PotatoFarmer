use std::fs::File;
use std::io::{Read, Write};
use macroquad::prelude::*;

use crate::constants::*;
use crate::types::*;
use super::Game;

impl Game {
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
            master_volume: self.sfx.volume,
            is_muted: self.sfx.is_music_muted,
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
        if let Ok(_file) = File::open(SAVE_FILE) {
            // Re-open as bytes
            if let Ok(mut f2) = File::open(SAVE_FILE) {
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
                            self.sfx.volume = data.master_volume;
                            self.sfx.is_music_muted = data.is_muted;
                            self.sfx.set_volume(data.master_volume);
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
                                    step_timer:    rand::gen_range(0.0_f32, 0.3_f32),
                                    talk_timer:    rand::gen_range(2.0_f32, 8.0_f32),
                                    search_cooldown: 0.0,
                                });
                            }
                            self.turrets.clear();
                            for (x, y, z) in data.turret_positions {
                                self.turrets.push(Turret { position: vec3(x, y, z), fire_cooldown: 0.0, angle: 0.0 });
                            }
                            self.iron_domes.clear();
                            for (x, y, z) in data.iron_dome_positions {
                                self.iron_domes.push(IronDome { position: vec3(x, y, z), cooldown: 0.0, angle: 0.0 });
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
}
