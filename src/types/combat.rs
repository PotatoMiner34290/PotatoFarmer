use macroquad::prelude::Vec3;
pub struct Turret{ pub position:Vec3, pub fire_cooldown:f32 }
pub struct IronDome{ pub position:Vec3, pub cooldown:f32 }
pub struct IronDomeMissile{ pub position:Vec3, pub target_pos:Vec3, pub speed:f32, pub life:f32 }
pub struct BulletParticle{ pub position:Vec3, pub velocity:Vec3, pub life:f32 }
pub struct RebelBullet{ pub position:Vec3, pub velocity:Vec3, pub life:f32 }
pub struct Rebel{ pub position:Vec3, pub target_cell:Option<(usize,usize)>, pub speed:f32, pub hp:f32, pub max_hp:f32, pub alive:bool, pub facing:f32, pub anim_timer:f32, pub raiding_timer:f32, pub shoot_cooldown:f32 }
pub struct GunBoat{ pub position:Vec3, pub target_z:f32, pub hp:f32, pub max_hp:f32, pub disembarked:bool, pub disembark_timer:f32, pub alive:bool }
pub struct AirEvent{ pub active:bool, pub timer:f32, pub fly_time:f32, pub bomber_pos:Vec3, pub jet1_pos:Vec3, pub jet2_pos:Vec3, pub bullets:Vec<BulletParticle> }
pub struct ThiefChild{ pub position:Vec3, pub target_cell:Option<(usize,usize)>, pub speed:f32, pub fleeing:bool, pub alive:bool, pub facing:f32, pub anim_timer:f32, pub harvesting_timer:f32, pub hp:f32, pub max_hp:f32, pub has_stolen:bool, pub flee_target:Vec3, pub steal_count:u8 }
