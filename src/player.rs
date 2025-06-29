use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use crate::{COL_ENEMY, COL_PLAYER};
use crate::game_entity::{ColliderInfo, GameEntity};

#[derive(Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub color: Color,
    pub layer: u8,
    pub mask: u8,
    pub angle:f32,
}

impl GameEntity for Player {
    fn draw(&self, graphics: &mut Graphics2D) {
        let (sin, cos) = self.angle.sin_cos();
        let outer_radius = self.radius + 6.0;
        let thickness = 8.0;
        graphics.draw_line(self.pos,self.pos+ Vec2::new(cos *outer_radius, sin * outer_radius),thickness, Color::CYAN);
        graphics.draw_circle((self.pos.x, self.pos.y), self.radius, self.color);
    }

    fn update(&mut self, dt: f32) -> bool {
        self.pos = self.pos + self.vel * dt;
        true
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
        self.radius -= 1.0;
    }
}

impl Player {
    pub fn new(pos: Vec2, radius: f32) -> Self {
        Player {
            pos,
            vel: Vec2::ZERO,
            radius,
            color: Color::WHITE,
            layer: COL_PLAYER,
            mask: COL_ENEMY,
            angle: 0.0,
        }
    }
}