extern crate glutin_window;
extern crate graphics;
extern crate nanorand;
extern crate opengl_graphics;
extern crate piston;

use apple::Apple;
use graphics::color::WHITE;
use graphics::rectangle::{self};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{Button, Key};
use rusty_audio::Audio;

#[derive(PartialEq)]
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
    pub direction: Directions,
    pub sounds: Audio,
    pub score: u64,
    pub tail: Vec<(f64, f64)>,
}

impl Snake {
    pub fn new(opengl: OpenGL, sounds: Audio, initial_x: f64, initial_y: f64) -> Snake {
        let size = 20.0;
        let snake_block_count = 4;

        let tail = (1..snake_block_count)
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

    fn verify_collision_with_body(&mut self) -> bool {
        let snake_head = (self.x, self.y);
        let snake_tail = &self.tail;

        snake_tail
            .iter()
            .any(|tail_block| *tail_block == snake_head)
    }

    fn verify_collision_with_wall(&mut self, args: &RenderArgs) -> bool {
        let (width, height) = (args.window_size[0], args.window_size[1]);
        let (snake_head_x_plus_size, snake_head_y_plus_size) = (self.x, self.y);

        if snake_head_x_plus_size >= width
            || self.x <= (-1.0 * self.size)
            || snake_head_y_plus_size >= height
            || self.y <= (-1.0 * self.size)
        {
            return true;
        }

        return false;
    }

    fn snake_moviment(&mut self) {
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
        let border_width = 1.0;

        let square_snake = rectangle::square(self.x, self.y, self.size);

        self.gl.draw(render_args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(WHITE, square_snake, transform, gl);
        });

        for (x, y) in self.tail.iter() {
            let square_snake_body_border = graphics::rectangle::square(
                *x - border_width,
                *y - border_width,
                self.size + border_width * 2.0,
            );
            let square_snake_body = graphics::rectangle::square(*x, *y, self.size);

            self.gl.draw(render_args.viewport(), |c, gl| {
                let transform = c.transform;

                graphics::rectangle(
                    graphics::color::BLACK,
                    square_snake_body_border,
                    transform,
                    gl,
                );
                graphics::rectangle(graphics::color::WHITE, square_snake_body, transform, gl);
            });
        }

        let collided_with_the_wall = self.verify_collision_with_wall(render_args);
        let verify_collision_with_body = self.verify_collision_with_body();

        if collided_with_the_wall || verify_collision_with_body {
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
            let (last_tail_x_coordinate, last_tail_y_coordinate) = *self.tail.last().unwrap();

            self.tail
                .push((last_tail_x_coordinate, last_tail_y_coordinate));

            apple.generate_new_apple(window_dimensions)
        }
    }

    pub fn keyboard_pressed(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                if self.direction != Directions::Down {
                    self.direction = Directions::Up;
                }
            }
            Button::Keyboard(Key::Down) => {
                if self.direction != Directions::Up {
                    self.direction = Directions::Down;
                }
            }
            Button::Keyboard(Key::Left) => {
                if self.direction != Directions::Right {
                    self.direction = Directions::Left;
                }
            }
            Button::Keyboard(Key::Right) => {
                if self.direction != Directions::Left {
                    self.direction = Directions::Right;
                }
            }
            _ => {}
        }
    }
}
