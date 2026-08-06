use macroquad::prelude::Vec3;
pub struct Farmer{ pub grid_x:i32, pub grid_z:i32, pub position:Vec3, pub facing:f32, pub plowing:bool, pub step_cooldown:f32, pub hp:f32, pub max_hp:f32 }
pub struct AiSlave{ pub position:Vec3, pub target_cell:Option<(usize,usize)>, pub action_timer:f32, pub anim_timer:f32, pub facing:f32 }
