mod particle;

use std::f32::consts::TAU;
use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window, dimen::Vec2};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};

use rand::random;
use speedy2d::dimen::Vector2;
use speedy2d::shape::Rectangle;
use speedy2d::time::Stopwatch;

use particle::Particle;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 960.0;

struct MyWindowHandler {
    timer: Stopwatch,
    frame_time: f64,

    particles: Vec<Particle>,
    background_color: Color,
    background_rect: Rectangle,
}

impl MyWindowHandler {
    pub fn new() -> Self {
        let particles = Vec::new();

        let timer = Stopwatch::new().unwrap();

        MyWindowHandler {
            frame_time: timer.secs_elapsed(),
            timer,
            particles,
            background_color: Color::from_int_rgba(0, 0, 0, 10),
            background_rect: Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(WIDTH, HEIGHT)),
        }
    }

    fn spawn_particles(&mut self, num_particles: u32) {
        let color = Color::from_rgb(0.0, 0.0, 1.0);
        for _ in 0..num_particles {
            let angle = random::<f32>() * TAU;
            let speed = random::<f32>() * 500.0;

            let vel = Vec2::new(angle.cos() * speed, angle.sin() * speed);

            let retain = |particle: &Particle| -> bool {
                particle.color.a() > 0.01
            };

            let particle = Particle::new(10.0, color,
                                         Vec2::new(WIDTH / 2.0, HEIGHT / 2.0),
                                         vel,
                                         0.99,
                                         0.9,
                                         retain);
            self.particles.push(particle);
        }
    }

    fn frame_time(&mut self) -> f32 {
        let old = self.frame_time;
        self.frame_time = self.timer.secs_elapsed();
        (self.frame_time - old) as f32
    }
}

impl WindowHandler for MyWindowHandler
{
    fn on_start(&mut self, _helper: &mut WindowHelper<()>, _info: WindowStartupInfo) {
        self.frame_time();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        let dt = self.frame_time();

        // auto spawn more particles
        self.spawn_particles(250);

        // set the window title
        helper.set_title(format!("Color Bang! FPS: {:.0} Particles: {}", 1.0 / dt, self.particles.len()));

        // set the background
        graphics.draw_rectangle(self.background_rect.clone(), self.background_color);

        // render content
        self.particles.retain_mut(|particle| {
            particle.draw(graphics);
            particle.update(dt)
        });

        // draw next frame
        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.spawn_particles(10);
            }
            MouseButton::Middle => {
                self.spawn_particles(1000);
            }
            MouseButton::Right => {
                self.spawn_particles(100);
            }
            MouseButton::Other(_) => {}
        }
    }
}

fn main() {
    let window = Window::new_centered("Title", (WIDTH as u32, HEIGHT as u32)).unwrap();
    window.run_loop(MyWindowHandler::new());
}