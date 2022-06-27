extern crate glutin_window;
extern crate graphics;
extern crate nanorand;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{Button, Key};
use rusty_audio::Audio;

use apple::Apple;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}



pub struct Snake {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub velocity: f64,
    pub direction: Directions,
    pub sounds: Audio,
    pub score: u64,
}

impl Snake {
    pub fn new(opengl: OpenGL, sounds: Audio, initial_x: f64, initial_y: f64) -> Snake {
        
        let snake = Snake {
            gl: GlGraphics::new(opengl),
            x: initial_x,
            y: initial_y,
            size: 20.0,
            velocity: 1.0,
            direction: Directions::Right,
            sounds,
            score: 0,
        };

        return snake;
    }

    fn verify_collision_with_wall(&mut self, args: &RenderArgs) -> bool {
        let (width, height) = (args.window_size[0], args.window_size[1]);

        if self.x + self.size >= width as f64
            || self.x < 0.0
            || self.y + self.size >= height as f64
            || self.y < 0.0
        {
            return true;
        }

        return false;
    }

    fn change_direction(&mut self) {
        match self.direction {
            Directions::Right => self.x += self.velocity,
            Directions::Left => self.x -= self.velocity,
            Directions::Up => self.y -= self.velocity,
            Directions::Down => self.y += self.velocity,
        }
    }

    fn collide_with_apple(&mut self, apple: &Apple) -> bool {
        let distance = (self.x - apple.x).powi(2) + (self.y - apple.y).powi(2);

        if distance < (self.size + apple.size).powi(2) / 2.0 {
            return true;
        }

        return false;
    }

    pub fn render(&mut self, render_args: &RenderArgs) -> bool {
        let square_snake = graphics::rectangle::square(self.x, self.y, self.size as f64);
        self.gl.draw(render_args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(graphics::color::WHITE, square_snake, transform, gl);
        });

        let collided_with_the_wall = self.verify_collision_with_wall(render_args);

        if collided_with_the_wall {
            println!("Game Over");
            println!("SEUS PONTOS: {}", self.score);
            return true;
        } else {
            return false;
        }
    }

    pub fn update(&mut self, _update_args: &UpdateArgs, apple: &mut Apple, window_dimensions: (f64, f64)) {
        self.change_direction();
        if self.collide_with_apple(apple) {

            self.sounds.play("bite");
            self.velocity += 0.1;
            self.score += 1;

            apple.generate_new_apple(window_dimensions)
        }
    }

    pub fn pressed(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => self.direction = Directions::Up,
            Button::Keyboard(Key::Down) => self.direction = Directions::Down,
            Button::Keyboard(Key::Left) => self.direction = Directions::Left,
            Button::Keyboard(Key::Right) => self.direction = Directions::Right,
            _ => {}
        }
    }
}
