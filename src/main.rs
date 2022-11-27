mod particle;
mod sound;
mod enemy;
mod player;
mod bullet;
mod game_entity;

use speedy2d::color::Color;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::font::TextLayout;
use speedy2d::dimen::{Vec2, Vector2};
use speedy2d::font::{Font, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::time::Stopwatch;
use crate::bullet::Bullet;

use crate::particle::Particle;
use crate::enemy::Enemy;
use crate::game_entity::{collide, GameEntity};
use crate::player::Player;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 960.0;

const COL_PLAYER: u8 = 0b00000001;
const COL_ENEMY: u8 = 0b00000010;
const COL_BULLET: u8 = 0b00000100;

struct MyWindowHandler {
    timer: Stopwatch,
    frame_time: f64,
    mouse_pos: Vec2,

    font: Font,
    sound: sound::SoundPlayer,

    level: u32,
    super_bang: u32,
    charged_super_bang: u32,
    charging: bool,

    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    particles: Vec<Particle>,
    background_color: Color,
    background_rect: Rectangle,
}

impl MyWindowHandler {
    pub fn new() -> Self {
        let enemies = Vec::new();
        let bullets = Vec::new();

        let particles = Vec::new();
        let timer = Stopwatch::new().unwrap();

        let font = Font::new(include_bytes!("../assets/Nasa21-l23X.ttf")).unwrap();

        MyWindowHandler {
            mouse_pos: Vec2::ZERO,
            frame_time: timer.secs_elapsed(),
            sound: sound::SoundPlayer::new(),
            timer,
            font,
            level: 1,
            super_bang: 0,
            charged_super_bang: 0,
            charging: false,
            player: Player::new(Vec2::new(WIDTH / 2.0, HEIGHT / 2.0), 20.0),
            enemies,
            bullets,
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

    fn display_text(&self, graphics: &mut Graphics2D, text: &str, pos: Vec2) {
        let formatted_text_block = self.font.layout_text(text, 32.0, TextOptions::new());
        graphics.draw_text((pos.x, pos.y), Color::WHITE, &formatted_text_block);
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

        if self.enemies.len() == 0 && self.bullets.len() == 0 {
            Enemy::spawn_n(&mut self.enemies, 1 * self.level, &self.player.pos);
            self.super_bang = self.super_bang + 4 * self.level;
            self.level += 1;
        }

        if self.charging && self.super_bang > 0 {
            self.super_bang -= 1;
            self.charged_super_bang += 10;
        }

        // set the window title
        helper.set_title(format!("Color Bang! FPS: {:.0} Particles: {}, Enemies: {}, Bullets: {}", 1.0 / dt, self.particles.len(), self.enemies.len(), self.bullets.len()));

        // set the background
        graphics.draw_rectangle(self.background_rect.clone(), self.background_color);

        // render player
        self.player.draw(graphics);
        self.player.update(dt);

        // enemies and bullets
        self.enemies.retain_mut(|enemy: &mut Enemy| {
            enemy.draw(graphics);

            if collide(enemy, &self.player) {
                self.sound.play();
                enemy.deal_damage();
                self.player.deal_damage();
                Particle::spawn_particles(&mut self.particles, 10, 500.0, self.player.color, enemy.pos);
            }

            self.bullets.retain_mut(|bullet: &mut Bullet| {
                return if collide(enemy, bullet) {
                    self.sound.play();
                    enemy.deal_damage();
                    bullet.deal_damage();
                    Particle::spawn_particles(&mut self.particles, 80, 500.0, enemy.color, enemy.pos);
                    false
                } else {
                    true
                };
            });

            enemy.update(dt)
        });

        self.bullets.retain_mut(|bullet: &mut Bullet| {
            if bullet.health > 0 {
                bullet.draw(graphics);
            }

            bullet.update(dt)
                && bullet.pos.x > 0.0 && bullet.pos.x < WIDTH
                && bullet.pos.y > 0.0 && bullet.pos.y < HEIGHT
        });

        // render particles
        self.particles.retain_mut(|particle| {
            particle.draw(graphics);
            particle.update(dt)
        });

        self.display_text(graphics, &format!("Level {}, Health: {}", self.level, self.player.radius), Vec2::new(20.0, 50.0));
        self.display_text(graphics, &format!("Super Bangs {}, Charged: {}", self.super_bang, self.charged_super_bang), Vec2::new(20.0, 90.0));



        // draw next frame
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = position;
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        if let MouseButton::Left = button {
            self.sound.play();
            let vel = (self.mouse_pos - self.player.pos).normalize().unwrap() * 200.0;
            self.bullets.push(Bullet::new(self.player.pos, vel, 5.0));
        } else if let MouseButton::Right = button {
            self.charging = true;
        }
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        if let MouseButton::Right = button {
            self.charging = false;
            self.sound.play();

            Bullet::super_bang(&mut self.bullets, self.charged_super_bang.max(10), self.player.pos);
            self.charged_super_bang = 0;
        }
    }
}

fn main() {
    let window = Window::new_centered("Title", (WIDTH as u32, HEIGHT as u32)).unwrap();
    window.run_loop(MyWindowHandler::new());
}