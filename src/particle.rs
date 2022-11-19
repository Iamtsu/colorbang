use speedy2d::dimen::Vec2;
use speedy2d::color::Color;
use speedy2d::{Graphics2D};


#[derive(Copy, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub drag_friction: f32,
    pub alpha_decay: f32,
    pub radius: f32,
    pub color: Color,
    pub retain: fn(&Self) -> bool,
}

impl Particle {
    pub fn new(radius: f32, color: Color, pos: Vec2, vel: Vec2, drag_friction: f32, alpha_decay: f32, retain: fn(&Particle) -> bool) -> Self {
        Particle {
            pos,
            vel,
            drag_friction,
            alpha_decay,
            radius,
            color,
            retain,
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle((self.pos.x, self.pos.y), self.radius, self.color);
    }


    pub fn update(&mut self, dt: f32) -> bool {
        if self.alpha_decay < 1.0 {
            let alpha = self.color.a() * self.alpha_decay;
            self.color = Color::from_rgba(self.color.r(), self.color.g(), self.color.b(), alpha)
        }

        self.vel = self.vel * self.drag_friction;
        self.pos = self.pos + self.vel * dt;

        (self.retain)(self)
    }
}