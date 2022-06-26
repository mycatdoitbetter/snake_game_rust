extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate nanorand;

use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs};
use nanorand::{Rng, WyRand};

pub struct Apple {
  pub gl: GlGraphics,
  pub x: f64,
  pub y: f64,
  pub size: f64,
}

impl Apple {
 pub fn render (&mut self, render_args: &RenderArgs) {

      let square_apple = graphics::rectangle::square(self.x, self.y, self.size as f64);

      self.gl.draw(render_args.viewport(), |c, gl| {
          let transform = c.transform;

          graphics::rectangle(graphics::color::RED, square_apple, transform, gl);
      });
  }

 pub fn generate_new_apple(&mut self){
    let mut rng = WyRand::new();

    // TODO: GET WINDOW DIMENSIONS
    self.x = rng.generate_range(1_u64..=620) as f64;
    self.y = rng.generate_range(1_u64..=460) as f64;
  }
}
