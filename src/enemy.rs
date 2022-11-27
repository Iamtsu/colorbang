use std::f32::consts::TAU;
use rand::random;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use crate::game_entity::{ColliderInfo, GameEntity};
use crate::{COL_BULLET, COL_ENEMY, COL_PLAYER};

#[derive(Clone)]
pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
    pub layer:u8,
    pub mask: u8,
}

impl GameEntity for Enemy {
    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle((self.pos.x, self.pos.y), self.radius, self.color);
    }

    fn update(&mut self, dt: f32) -> bool {
        self.pos = self.pos + self.vel * dt;
        self.radius > 3.0
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

    fn deal_damage(&mut self) {
        self.radius = (self.radius - 5.0).max(0.0);
    }
}

impl Enemy {
    pub fn spawn(target: &Vec2) -> Self {
        let angle = random::<f32>() * TAU;
        let dist = (random::<f32>() * 100.0) + 300.0;
        let radius = random::<f32>() * 20.0 + 10.0;
        let color = Color::from_rgb(random(), random(), random());

        let pos = Vec2::new(target.x + angle.cos() * dist, target.y + angle.sin() * dist);
        let vel: Vec2 = (target - pos).normalize().unwrap() * (random::<f32>() * 30.0 + 20.0);

        Enemy {
            pos,
            vel,
            radius,
            color,
            layer: COL_ENEMY,
            mask: COL_PLAYER | COL_BULLET
        }
    }

    pub fn spawn_n(enemies: &mut Vec<Enemy>, num: u32, target: &Vec2) {
        for _ in 0..num {
            enemies.push(Enemy::spawn(target));
        }
    }
}