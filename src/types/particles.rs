use macroquad::prelude::{Vec3,Color};
pub struct DirtParticle{ pub position:Vec3, pub velocity:Vec3, pub life:f32, pub color:Color }
pub struct SparkleParticle{ pub position:Vec3, pub velocity:Vec3, pub life:f32, pub max_life:f32, pub color:Color }
pub struct SmokeParticle{ pub position:Vec3, pub velocity:Vec3, pub life:f32, pub max_life:f32, pub size:f32, pub color:Color }

