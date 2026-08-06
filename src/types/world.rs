use macroquad::prelude::Vec3;
#[derive(Clone,Copy)] pub struct HouseBounds{ pub center:Vec3, pub min_x:f32, pub max_x:f32, pub min_z:f32, pub max_z:f32, pub style:usize }
pub struct CameraState{ pub position:Vec3, pub target:Vec3 }
impl CameraState{ #[inline(always)] pub fn is_in_view(&self, p:Vec3, r:f32)->bool{ let (min_x,max_x)=(self.target.x-30.0-r, self.target.x+30.0+r); let (min_z,max_z)=(self.target.z-25.0-r, self.target.z+25.0+r); p.x>=min_x && p.x<=max_x && p.z>=min_z && p.z<=max_z } }
#[inline(always)] pub fn cell_hash(gx:usize, gz:usize, i:u32)->f32{ let mut h=(gx as u32).wrapping_mul(374761393)^(gz as u32).wrapping_mul(668265263)^i.wrapping_mul(2246822519); h=(h^(h>>13)).wrapping_mul(1274126177); h as f32 / u32::MAX as f32 }
