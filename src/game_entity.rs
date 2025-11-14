use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;

pub fn collide(e1: &impl GameEntity, e2: &impl GameEntity) -> bool {
    let c1 = e1.collider_info();
    let c2 = e2.collider_info();

    let can_collide = (*c1.layer & *c2.mask) | (*c1.mask & *c2.layer);
    if can_collide != 0 {
        let dist = (c1.pos - c2.pos).magnitude();
        dist <= (*c1.radius + *c2.radius)
    } else {
        false
    }
}

#[inline]
pub fn impulse(v1: &Vec2, m1: f32, v2: &Vec2, m2: f32) -> Vec2 {
    (v1 * m1 + (v2  - v1) * m2) / (m1 + m2)
}


pub struct ColliderInfo<'a> {
    pub mask: &'a u8,
    pub layer: &'a u8,
    pub pos: &'a Vec2,
    pub radius: &'a f32,
}

pub trait GameEntity {
    fn draw(&self, graphics: &mut Graphics2D);
    fn update(&mut self, dt: f32) -> bool;
    fn collider_info(&self) -> ColliderInfo<'_>;
    fn deal_damage(&mut self, other_vel: &Vec2, other_mass: f32);
}


