use std::f32::consts::TAU;
use rand::random;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use crate::{COL_BULLET, COL_ENEMY};
use crate::game_entity::{ColliderInfo, GameEntity};

#[derive(Clone)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
    pub health: i32,
    pub layer: u8,
    pub mask: u8,
}

impl GameEntity for Bullet {
    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle((self.pos.x, self.pos.y), self.radius, self.color);
    }

    fn update(&mut self, dt: f32) -> bool {
        self.pos = self.pos + self.vel * dt;
        self.health -= 1;
        self.health > 0
    }

    fn collider_info(&self) -> ColliderInfo {
        ColliderInfo {
            mask: &self.mask,
            layer : & self.layer,
            pos: &self.pos,
            vel: &self.vel,
            radius: &self.radius,
        }
    }

    fn deal_damage(&mut self, _other_vel: &Vec2, _other_mass: f32) {
        self.health = 0;
    }
}

impl Bullet {
    pub fn new(pos: Vec2, vel: Vec2, radius: f32) -> Self {
        Bullet {
            pos,
            vel,
            radius,
            color: Color::WHITE,
            health: 600,
            layer: COL_BULLET,
            mask: COL_ENEMY,
        }
    }

    pub fn super_bang(bullets: &mut Vec<Bullet>, num: u32, pos: Vec2) {
        for _ in 0..num {
            let angle = random::<f32>() * TAU;
            let speed = random::<f32>() * 200.0 + 100.0;

            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

            let bullet = Bullet {
                pos,
                vel,
                radius: 2.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                health: 200,
                layer: COL_BULLET,
                mask: COL_ENEMY,
            };

            bullets.push(bullet);
        }
    }
}