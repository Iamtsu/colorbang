use std::f32::consts::TAU;
use std::fmt;
use std::fmt::Formatter;
use speedy2d::dimen::Vec2;
use speedy2d::color::Color;
use speedy2d::{Graphics2D};
use rand::random;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub drag_friction: f32,
    pub alpha_decay: f32,
    pub radius: f32,
    pub color: Color,
}

impl fmt::Debug for Particle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Particle")
            .field("pos", &self.pos)
            .field("vel", &self.vel)
            .field("drag_friction", &self.drag_friction)
            .field("alpha_decay", &self.alpha_decay)
            .field("radius", &self.radius)
            .field("color", &self.color)
            .finish()
    }
}

impl Particle {
    pub fn new(radius: f32, color: Color, pos: Vec2, vel: Vec2, drag_friction: f32, alpha_decay: f32) -> Self {
        Particle {
            pos,
            vel,
            drag_friction,
            alpha_decay,
            radius,
            color,
        }
    }

    fn retain(&self) -> bool {
        self.radius > 0.01
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_circle((self.pos.x, self.pos.y), self.radius, self.color);
    }


    pub fn update(&mut self, dt: f32) -> bool {
        if self.alpha_decay < 1.0 {
            let mut alpha = self.color.a() * self.alpha_decay;
            if alpha < 0.01 {
                alpha = 0.0;
            }
            self.color = Color::from_rgba(self.color.r(), self.color.g(), self.color.b(), alpha)
        }

        self.radius -= 0.1;

        self.vel = self.vel * self.drag_friction;
        self.pos = self.pos + self.vel * dt;

        self.retain()
    }

    pub fn spawn_particles(particles: &mut Vec<Particle>, num_particles: u32, speed: f32, color: Color, pos: Vec2) {
        for _ in 0..num_particles {
            let angle = random::<f32>() * TAU;
            let speed = random::<f32>() * speed;

            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

            let particle = Particle::new(5.0, color,
                                         pos,
                                         vel,
                                         0.99,
                                         1.0);

            particles.push(particle);
        }
    }
}