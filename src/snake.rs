extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate nanorand;

use opengl_graphics::{GlGraphics};
use piston::{Key, Button };
use piston::input::{RenderArgs, UpdateArgs};

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
}

impl Snake {
  fn verify_collision_with_wall(&mut self, args: &RenderArgs ) -> bool {
      let (width, height) = (args.window_size[0], args.window_size[1]);
        
      if 
      self.x + self.size  >= width as f64 ||
      self.x  < 0.0 || 
      self.y + self.size >= height as f64 ||
      self.y  < 0.0 {
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
  pub fn render (&mut self, render_args: &RenderArgs) -> bool {
      let square_snake = graphics::rectangle::square(self.x, self.y, self.size as f64);
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
  
  pub fn update(&mut self, _update_args: &UpdateArgs) {
   self.change_direction();
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
