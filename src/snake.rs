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

// TODO: Move score and sound to game struct
pub struct Snake {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub direction: Directions,
    pub sounds: Audio,
    pub score: u64,
    pub tail: Vec<(f64, f64)>,
}

impl Snake {
    pub fn new(opengl: OpenGL, sounds: Audio, initial_x: f64, initial_y: f64) -> Snake {
        let size = 20.0;
        let snake_tail_block_count = 3;

        let tail = (0..snake_tail_block_count)
            .map(|snake_tail_block| (initial_x - (snake_tail_block as f64 * size), initial_y))
            .collect();

        let snake = Snake {
            gl: GlGraphics::new(opengl),
            x: initial_x,
            y: initial_y,
            size,
            direction: Directions::Right,
            sounds,
            score: 0,
            tail,
        };

        return snake;
    }

    fn verify_collision_with_wall(&mut self, args: &RenderArgs) -> bool {
        let width = args.window_size[0];
        let height = args.window_size[1];

        // TODO: Refactor this to a better readable code
        if self.x + self.size >= width
            || self.x <= 0.0
            || self.y + self.size >= height
            || self.y <= 0.0
        {
            return true;
        }

        return false;
    }

    // TODO: Add a method to verify collision with the snake tail
    // TODO: Avoid snake to go back to the same direction
    fn snake_moviment(&mut self) {
        // TODO: Refactor this to a safer tail movement, maybe a method on impl Snake
        let mut tail = self.tail.clone();
        tail.insert(0, (self.x, self.y));
        tail.pop();
        self.tail = tail;

        match self.direction {
            Directions::Right => self.x += self.size,
            Directions::Left => self.x -= self.size,
            Directions::Up => self.y -= self.size,
            Directions::Down => self.y += self.size,
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
        // TODO: Refactor this to a better readable code
        let square_snake = graphics::rectangle::square(self.x, self.y, self.size);

        self.gl.draw(render_args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(graphics::color::WHITE, square_snake, transform, gl);
        });

        // TODO: Look up for others ways to map the tail and improve the readability
        for (x, y) in self.tail.iter() {
            let square_snake_body = graphics::rectangle::square(*x, *y, self.size);

            self.gl.draw(render_args.viewport(), |c, gl| {
                let transform = c.transform;

                graphics::rectangle(graphics::color::WHITE, square_snake_body, transform, gl);
            });
        }

        let collided_with_the_wall = self.verify_collision_with_wall(render_args);

        if collided_with_the_wall {
            println!("Game Over 🕹 - score: {}", self.score);
            return true;
        } else {
            return false;
        }
    }

    pub fn update(
        &mut self,
        _update_args: &UpdateArgs,
        apple: &mut Apple,
        window_dimensions: (f64, f64),
    ) {
        self.snake_moviment();
        if self.collide_with_apple(apple) {
            self.sounds.play("bite");
            self.score += 1;
            // FIXME: Verify this warning about borrowing tail coordinates
            let (mut last_x, mut last_y) = self.tail.last().unwrap();
            
            self.tail.push((last_x, last_y));

            apple.generate_new_apple(window_dimensions)
        }
    }

    pub fn keyboard_pressed(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => self.direction = Directions::Up,
            Button::Keyboard(Key::Down) => self.direction = Directions::Down,
            Button::Keyboard(Key::Left) => self.direction = Directions::Left,
            Button::Keyboard(Key::Right) => self.direction = Directions::Right,
            _ => {}
        }
    }
}
