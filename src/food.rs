extern crate rand;

use piston::input::*;
use opengl_graphics::{GlGraphics};

use crate::snake::{Snake};

pub struct Food {
    pub x: u32,
    pub y: u32,
}

impl Food {
    pub fn update(&mut self, snake: &Snake) -> bool {
        let front = snake.parts.front().unwrap();

        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {

        const BLACK: [f32; 4] = [0.0, 0.23, 1.0, 0.6];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl);
        })
    }
}