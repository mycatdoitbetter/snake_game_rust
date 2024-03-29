extern crate glutin_window;
extern crate graphics;
extern crate nanorand;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rusty_audio;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonEvent, ButtonState, EventLoop};
use piston_window::*;
use rusty_audio::Audio;

pub mod apple;
use apple::Apple;

pub mod snake;
use snake::Snake;

struct Game {
    gl: GlGraphics,
    window_dimensions: (f64, f64),
    snake: Snake,
    apple: Apple,
}

// TODO: Mock general values, like the window size, the snake size, the apple size, etc.
impl Game {
    fn new(opengl: OpenGL, screen_width: f64, screen_height: f64) -> Game {
        let mut sounds = Audio::new();

        sounds.add("bite", "./src/assets/bite.wav");

        let snake = Snake::new(opengl, sounds, screen_width / 2.0, screen_height / 2.0);
        let apple = Apple::new(opengl, screen_width, screen_height);

        Game {
            gl: GlGraphics::new(opengl),
            snake,
            apple,
            window_dimensions: (screen_width, screen_height),
        }
    }

    fn render(&mut self, render_args: &RenderArgs) -> bool {
        self.gl.draw(render_args.viewport(), |_c, gl| {
            graphics::clear(color::BLACK, gl);
        });

        self.apple.render(render_args);
        let still_alive = !self.snake.render(render_args);

        return still_alive;
    }

    fn update(&mut self, update_args: &UpdateArgs) {
        self.snake
            .update(update_args, &mut self.apple, self.window_dimensions);
    }

    fn keyboard_pressed(&mut self, button: Button) {
        self.snake.keyboard_pressed(button);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let (window_width, window_height) = (640.0, 480.0);

    let mut events = Events::new(EventSettings::new());

    let mut window: PistonWindow = WindowSettings::new("snake game", [window_width, window_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(opengl, window_width, window_height);

    events.set_ups(8);

    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            if !game.render(&render_args) {
                break;
            }
        }

        if let Some(update_args) = event.update_args() {
            game.update(&update_args);
        }

        if let Some(button) = event.button_args() {
            if button.state == ButtonState::Press {
                game.keyboard_pressed(button.button);
            }
        }
    }
}
