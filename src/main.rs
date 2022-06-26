extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate nanorand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ Button, ButtonEvent, ButtonState, EventLoop };
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub mod apple;
use apple::Apple;

pub mod snake;
use snake::Snake;
use snake::Directions;

struct Game {
    gl: GlGraphics, 
    snake: Snake,
    apple: Apple,  
}

impl Game {
    fn render(&mut self, render_args: &RenderArgs) -> bool {
        self.gl.draw(render_args.viewport(), |_c, gl| {
            graphics::clear(
                graphics::color::BLACK,
                gl
            );
        });

        self.apple.render(render_args);
        let still_alive = !self.snake.render(render_args);
        
        return still_alive;
    }

    fn update(&mut self, update_args: &UpdateArgs) {
        self.snake.update(update_args, &mut self.apple);
    }

    fn pressed(&mut self, button: Button) {
        self.snake.pressed(button);
    }
  
}

fn main() {
    let opengl = OpenGL::V3_2;

    let (window_width, window_height) = (640, 480);

    let mut window: Window = WindowSettings::new("snake game", [window_width, window_height])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            gl: GlGraphics::new(opengl),
            size: 20.0,
            x: (window_width / 2) as f64,
            y: (window_height / 2) as f64,
            velocity: 1.5,
            direction: Directions::Right
        },
        apple: Apple {
            gl: GlGraphics::new(opengl),
            x: 200.0,
            y: 130.0,
            size: 20.0,
        }
    };

    let mut events = Events::new(EventSettings::new());
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
                game.pressed(button.button);
            }
        }
    }
}
