extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate nanorand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{Key, Button, ButtonEvent, ButtonState};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

#[derive(Clone)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    gl: GlGraphics, 
    snake: Snake,  
}

impl Game {


    fn render(&mut self, render_args: &RenderArgs) -> bool {
        self.gl.draw(render_args.viewport(), |_c, gl| {
            graphics::clear(
                graphics::color::BLACK,
                gl
            );
        });

        let still_alive = !self.snake.render(render_args);
        
        return still_alive;
    }

    fn update(&mut self, update_args: &UpdateArgs) {
        self.snake.update(update_args);
    }

    fn pressed(&mut self, button: Button) {
        self.snake.pressed(button);
    }
  
}

struct Snake {
    gl: GlGraphics,
    x: f64,
    y: f64,
    velocity: f64,
    direction: Directions,
}

 impl Snake {
    fn verify_collision_with_wall(&mut self, args: &RenderArgs ) -> bool {
        let (width, height) = (args.window_size[0], args.window_size[1]);

        if self.x >= width as f64 || self.x < 0.0 || self.y >= height as f64 || self.y < 0.0 {
            return true;
        }

        return false;
    }
    fn change_direction(&mut self){
        match self.direction {
            Directions::Right => self.x += self.velocity,
            Directions::Left => self.x -= self.velocity,
            Directions::Up => self.y -= self.velocity,
            Directions::Down => self.y += self.velocity,
        }
    }
    fn render (&mut self, render_args: &RenderArgs) -> bool {
        let square_snake = graphics::rectangle::square(self.x, self.y, 20.0);
        self.gl.draw(render_args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(graphics::color::WHITE, square_snake, transform, gl);
        });

        let collided_with_the_wall = self.verify_collision_with_wall(render_args);

        if collided_with_the_wall {
            println!("Game Over");
            return true;
        } else {
            return false;
        }
    }
    
    fn update(&mut self, _update_args: &UpdateArgs) {
     self.change_direction();
    }
 
    fn pressed(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => self.direction = Directions::Up,
            Button::Keyboard(Key::Down) => self.direction = Directions::Down,
            Button::Keyboard(Key::Left) => self.direction = Directions::Left,
            Button::Keyboard(Key::Right) => self.direction = Directions::Right,
            _ => {}
        }
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
            x: (window_width / 2) as f64,
            y: (window_height / 2) as f64,
            velocity: 1.5,
            direction: Directions::Right
        },
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
