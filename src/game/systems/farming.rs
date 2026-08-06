use crate::constants::*;
pub fn grid_to_world(gx:i32,gz:i32)->macroquad::prelude::Vec3{ macroquad::prelude::vec3(-FIELD_HALF + gx as f32*CELL + CELL/2.0, 0.0, -FIELD_HALF + gz as f32*CELL + CELL/2.0) }
