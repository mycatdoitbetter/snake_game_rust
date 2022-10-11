extern crate glutin_window;
extern crate graphics;
extern crate nanorand;
extern crate opengl_graphics;
extern crate piston;

use graphics::{
    clear,
    color::{self},
};
use nanorand::{Rng, WyRand};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::RenderArgs;

pub struct Apple {
    pub gl: GlGraphics,
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl Apple {
    pub fn new(opengl: OpenGL, screen_width: f64, screen_height: f64) -> Apple {
        let mut rand = WyRand::new();

        let apple = Apple {
            gl: GlGraphics::new(opengl),
            x: rand.generate_range(1_u64..=screen_width as u64) as f64,
            y: rand.generate_range(1_u64..=screen_height as u64) as f64,
            size: 20.0,
        };

        return apple;
    }
    pub fn render(&mut self, render_args: &RenderArgs) {
        let square_apple = graphics::rectangle::square(self.x, self.y, self.size);

        self.gl.draw(render_args.viewport(), |context, gl| {
            let transform = context.transform;
            clear(color::BLACK, gl);

            graphics::rectangle(color::RED, square_apple, transform, gl);
        });
    }

    // TODO: Prevent apple from spawning on the snake
    pub fn generate_new_apple(&mut self, window_dimensions: (f64, f64)) {
        let mut rand = WyRand::new();

        let (width, height) = (window_dimensions.0 - self.size, window_dimensions.1 - self.size);

        self.x = rand.generate_range(1_u64..=width as u64) as f64;
        self.y = rand.generate_range(1_u64..=height as u64) as f64;
    }
}
