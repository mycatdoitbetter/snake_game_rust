extern crate glutin_window;
extern crate graphics;
extern crate nanorand;
extern crate opengl_graphics;
extern crate piston;

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

            graphics::rectangle(graphics::color::RED, square_apple, transform, gl);
        });
    }
    pub fn generate_new_apple(&mut self, window_dimensions: (f64, f64)) {
        // TODO: Verify if the new apple is not being generated inside the snake or outside the screen
        let mut rand = WyRand::new();

        // TODO: Change the way to access the window dimensions tuple
        let (width, height) = (window_dimensions.0 - self.size, window_dimensions.1 - self.size);

        self.x = rand.generate_range(1_u64..=width as u64) as f64;
        self.y = rand.generate_range(1_u64..=height as u64) as f64;
    }
}
