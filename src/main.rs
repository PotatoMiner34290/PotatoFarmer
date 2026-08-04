use macroquad::prelude::*;

const GRID: usize = 10;
const CELL: f32 = 2.0;
const FIELD_HALF: f32 = GRID as f32 * CELL / 2.0;
const GROW_TIME: f32 = 18.0;
const MOVE_SPEED: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Grass,
    Plowed,
    Planted { growth: f32 },
}

struct DirtParticle {
    position: Vec3,
    velocity: Vec3,
    life: f32,
}

struct Farmer {
    position: Vec3,
    yaw: f32,
    plowing: bool,
}

struct Game {
    field: [[CellState; GRID]; GRID],
    farmer: Farmer,
    dirt: Vec<DirtParticle>,
    seeds: u32,
    potatoes: u32,
    action_cooldown: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            field: [[CellState::Grass; GRID]; GRID],
            farmer: Farmer {
                position: vec3(0.0, 0.0, 0.0),
                yaw: 0.0,
                plowing: false,
            },
            dirt: Vec::new(),
            seeds: 24,
            potatoes: 0,
            action_cooldown: 0.0,
        }
    }

    fn cell_center(grid_x: usize, grid_z: usize) -> Vec3 {
        vec3(
            -FIELD_HALF + grid_x as f32 * CELL + CELL / 2.0,
            0.0,
            -FIELD_HALF + grid_z as f32 * CELL + CELL / 2.0,
        )
    }

    fn world_to_cell(pos: Vec3) -> Option<(usize, usize)> {
        let local_x = pos.x + FIELD_HALF;
        let local_z = pos.z + FIELD_HALF;
        if local_x < 0.0 || local_z < 0.0 {
            return None;
        }
        let gx = (local_x / CELL) as usize;
        let gz = (local_z / CELL) as usize;
        if gx >= GRID || gz >= GRID {
            return None;
        }
        Some((gx, gz))
    }

    fn farmer_cell(&self) -> Option<(usize, usize)> {
        Self::world_to_cell(self.farmer.position)
    }

    fn spawn_dirt(&mut self, pos: Vec3) {
        for _ in 0..4 {
            self.dirt.push(DirtParticle {
                position: pos + vec3(0.0, 0.15, 0.0),
                velocity: vec3(
                    rand::gen_range(-2.0, 2.0),
                    rand::gen_range(3.0, 6.0),
                    rand::gen_range(-2.0, 2.0),
                ),
                life: rand::gen_range(0.6, 1.2),
            });
        }
    }

    fn plow_cell(&mut self, gx: usize, gz: usize) {
        if self.field[gx][gz] == CellState::Grass {
            self.field[gx][gz] = CellState::Plowed;
            self.spawn_dirt(Self::cell_center(gx, gz));
        }
    }

    fn plant_cell(&mut self, gx: usize, gz: usize) -> bool {
        if self.seeds == 0 {
            return false;
        }
        if self.field[gx][gz] == CellState::Plowed {
            self.field[gx][gz] = CellState::Planted { growth: 0.0 };
            self.seeds -= 1;
            return true;
        }
        false
    }

    fn harvest_cell(&mut self, gx: usize, gz: usize) -> bool {
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

    fn update(&mut self, dt: f32) {
        self.action_cooldown = (self.action_cooldown - dt).max(0.0);

        let mut move_dir = vec3(0.0, 0.0, 0.0);
        if is_key_down(KeyCode::W) {
            move_dir.z -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.z += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }

        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
            self.farmer.yaw = move_dir.x.atan2(move_dir.z);
            self.farmer.position += move_dir * MOVE_SPEED * dt;
        }

        let margin = 0.4;
        self.farmer.position.x = self
            .farmer
            .position
            .x
            .clamp(-FIELD_HALF + margin, FIELD_HALF - margin);
        self.farmer.position.z = self
            .farmer
            .position
            .z
            .clamp(-FIELD_HALF + margin, FIELD_HALF - margin);
        self.farmer.position.y = 0.0;

        self.farmer.plowing = is_key_down(KeyCode::Space);

        if self.farmer.plowing {
            if let Some((gx, gz)) = self.farmer_cell() {
                self.plow_cell(gx, gz);
            }
        }

        if is_key_pressed(KeyCode::E) && self.action_cooldown <= 0.0 {
            if let Some((gx, gz)) = self.farmer_cell() {
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

        for row in self.field.iter_mut() {
            for cell in row.iter_mut() {
                if let CellState::Planted { growth } = cell {
                    *growth = (*growth + dt / GROW_TIME).min(1.0);
                }
            }
        }

        for particle in self.dirt.iter_mut() {
            particle.velocity.y -= 12.0 * dt;
            particle.position += particle.velocity * dt;
            particle.life -= dt;
        }
        self.dirt.retain(|p| p.life > 0.0 && p.position.y > -0.5);
    }
}

fn soil_color(state: CellState) -> Color {
    match state {
        CellState::Grass => Color::from_rgba(70, 130, 55, 255),
        CellState::Plowed => Color::from_rgba(95, 60, 30, 255),
        CellState::Planted { .. } => Color::from_rgba(80, 50, 25, 255),
    }
}

fn draw_field(game: &Game) {
    for gx in 0..GRID {
        for gz in 0..GRID {
            let center = Game::cell_center(gx, gz);
            let state = game.field[gx][gz];
            let soil = soil_color(state);

            draw_cube(
                center + vec3(0.0, -0.08, 0.0),
                vec3(CELL * 0.96, 0.16, CELL * 0.96),
                None,
                soil,
            );

            if let CellState::Planted { growth } = state {
                draw_potato_plant(center, growth);
            }
        }
    }
}

fn draw_potato_plant(center: Vec3, growth: f32) {
    let stem_h = 0.15 + growth * 1.1;
    let stem_r = 0.06 + growth * 0.04;

    draw_cylinder(
        center + vec3(0.0, stem_h / 2.0 + 0.08, 0.0),
        stem_r,
        stem_r,
        stem_h,
        None,
        Color::from_rgba(40, 110, 35, 255),
    );

    if growth > 0.25 {
        let leaf = 0.12 + growth * 0.18;
        draw_sphere(
            center + vec3(-0.2, 0.35 + growth * 0.5, 0.0),
            leaf,
            None,
            Color::from_rgba(55, 150, 45, 255),
        );
        draw_sphere(
            center + vec3(0.2, 0.45 + growth * 0.55, 0.0),
            leaf,
            None,
            Color::from_rgba(45, 130, 40, 255),
        );
    }

    if growth > 0.55 {
        draw_sphere(
            center + vec3(0.0, 0.9 + growth * 0.4, 0.0),
            0.14 + growth * 0.08,
            None,
            Color::from_rgba(70, 170, 55, 255),
        );
    }

    if growth > 0.85 {
        let potato_color = Color::from_rgba(160, 120, 70, 255);
        draw_sphere(
            center + vec3(-0.15, 0.12, 0.12),
            0.12,
            None,
            potato_color,
        );
        draw_sphere(
            center + vec3(0.12, 0.1, -0.1),
            0.11,
            None,
            potato_color,
        );
    }
}

fn draw_farmer_3d(farmer: &Farmer) {
    let pos = farmer.position;
    let forward = vec3(farmer.yaw.sin(), 0.0, farmer.yaw.cos());
    let right = vec3(forward.z, 0.0, -forward.x);

    draw_cylinder(
        pos + vec3(0.0, 0.45, 0.0),
        0.22,
        0.22,
        0.9,
        None,
        Color::from_rgba(30, 80, 180, 255),
    );

    draw_sphere(
        pos + vec3(0.0, 1.05, 0.0),
        0.22,
        None,
        Color::from_rgba(230, 180, 140, 255),
    );

    draw_cylinder(
        pos - forward * 0.05 + vec3(0.0, 0.18, 0.0),
        0.1,
        0.1,
        0.55,
        None,
        DARKGRAY,
    );
    draw_cylinder(
        pos + forward * 0.05 + vec3(0.0, 0.18, 0.0),
        0.1,
        0.1,
        0.55,
        None,
        DARKGRAY,
    );

    draw_cylinder(
        pos + right * 0.35 + vec3(0.0, 0.55, 0.0),
        0.07,
        0.07,
        0.55,
        None,
        Color::from_rgba(230, 180, 140, 255),
    );

    if farmer.plowing {
        let plow_pos = pos + forward * 0.9 + vec3(0.0, 0.12, 0.0);
        draw_cube(
            plow_pos,
            vec3(0.5, 0.12, 0.35),
            None,
            Color::from_rgba(60, 60, 65, 255),
        );
        draw_line_3d(
            pos + right * 0.35 + vec3(0.0, 0.55, 0.0),
            plow_pos + vec3(0.0, 0.15, 0.0),
            DARKGRAY,
        );
    }
}

fn draw_scene(game: &Game) {
    let sky = Color::from_rgba(135, 200, 245, 255);
    clear_background(sky);

    let cam_target = game.farmer.position + vec3(0.0, 1.0, 0.0);
    let cam_pos = cam_target
        + vec3(
            -game.farmer.yaw.sin() * 10.0,
            9.0,
            -game.farmer.yaw.cos() * 10.0,
        );

    set_camera(&Camera3D {
        position: cam_pos,
        up: vec3(0.0, 1.0, 0.0),
        target: cam_target,
        fovy: 55.0,
        ..Default::default()
    });

    draw_grid(GRID as u32, CELL, Color::from_rgba(40, 40, 40, 80), GRAY);

    draw_field(game);

    for particle in &game.dirt {
        let alpha = (particle.life * 255.0) as u8;
        draw_sphere(
            particle.position,
            0.08,
            None,
            Color::from_rgba(120, 75, 35, alpha),
        );
    }

    draw_farmer_3d(&game.farmer);

    if let Some((gx, gz)) = game.farmer_cell() {
        let highlight = Game::cell_center(gx, gz) + vec3(0.0, 0.02, 0.0);
        draw_cube_wires(highlight, vec3(CELL * 0.98, 0.05, CELL * 0.98), YELLOW);
    }

    set_default_camera();
}

fn draw_hud(game: &Game) {
    draw_text("WASD - Move", 16.0, 28.0, 22.0, BLACK);
    draw_text("SPACE (hold) - Plow field", 16.0, 54.0, 22.0, BLACK);
    draw_text("E - Plant potato / Harvest", 16.0, 80.0, 22.0, BLACK);
    draw_text(
        &format!("Seeds: {}   Potatoes: {}", game.seeds, game.potatoes),
        16.0,
        112.0,
        26.0,
        DARKBLUE,
    );

    if let Some((gx, gz)) = game.farmer_cell() {
        let status = match game.field[gx][gz] {
            CellState::Grass => "Grass - hold SPACE to plow".to_string(),
            CellState::Plowed => "Plowed - press E to plant".to_string(),
            CellState::Planted { growth } if growth >= 1.0 => {
                "Ready! - press E to harvest".to_string()
            }
            CellState::Planted { growth } => {
                format!("Growing... {}%", (growth * 100.0) as u32)
            }
        };
        draw_text(&status, 16.0, 148.0, 22.0, DARKGREEN);
    }
}

#[macroquad::main("Nigga Farmer")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time();
        game.update(dt);
        draw_scene(&game);
        draw_hud(&game);
        next_frame().await;
    }
}
