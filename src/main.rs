mod particle;
mod sound;

use rand::random;
use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};

use speedy2d::dimen::{Vec2, Vector2};
use speedy2d::shape::Rectangle;
use speedy2d::time::Stopwatch;

use particle::Particle;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 960.0;

struct MyWindowHandler {
    timer: Stopwatch,
    frame_time: f64,
    mouse_pos: Vec2,

    sound: sound::SoundPlayer,

    particles: Vec<Particle>,
    background_color: Color,
    background_rect: Rectangle,
}

impl MyWindowHandler {
    pub fn new() -> Self {
        let particles = Vec::new();

        let timer = Stopwatch::new().unwrap();

        MyWindowHandler {
            mouse_pos: Vec2::ZERO,
            frame_time: timer.secs_elapsed(),
            sound: sound::SoundPlayer::new(),
            timer,
            particles,
            background_color: Color::from_int_rgba(0, 0, 0, 50),
            background_rect: Rectangle::new(Vector2::new(0.0, 0.0), Vector2::new(WIDTH, HEIGHT)),
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

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = position;
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        if let MouseButton::Left = button {
            self.sound.play();
            let color = Color::from_rgb(random(), random(), random());
            Particle::spawn_particles(&mut self.particles, 10, 500.0, color, self.mouse_pos);
        }
    }
}

fn main() {
    let window = Window::new_centered("Title", (WIDTH as u32, HEIGHT as u32)).unwrap();
    window.run_loop(MyWindowHandler::new());
}