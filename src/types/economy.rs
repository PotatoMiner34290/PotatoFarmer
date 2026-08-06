use macroquad::prelude::Vec3; use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize,Clone,Copy,PartialEq,Debug)] pub enum LootType{ BloodDiamonds, Cash, PantherStatue, Gold, Bullets, Minigun }
pub struct DroppedLoot{ pub loot_type:LootType, pub position:Vec3, pub amount:u32 }
pub struct CrashingBomber{ pub position:Vec3, pub velocity:Vec3, pub rotation:f32, pub rot_speed:f32, pub life:f32 }
pub struct MinigunBullet{ pub position:Vec3, pub velocity:Vec3, pub life:f32 }
