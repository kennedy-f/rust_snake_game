extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::input::*;
use opengl_graphics::{GlGraphics};

use crate::game::{Direction};
use std::collections::LinkedList;
pub struct Snake {
    pub gl: GlGraphics,
    pub width: u32,
    pub direction: Direction,
    pub parts: LinkedList<SnakePieces>,
}

#[derive(Clone)]
pub struct SnakePieces(pub u32, pub u32);

impl Snake {
    pub fn render(&mut self, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.parts
        .iter()
        .map(|p| SnakePieces(p.0 * self.width, p.1 * self.width))
        .map(|p| graphics::rectangle::square(p.0 as f64, p.1 as f64, self.width as f64))
        .collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter().for_each(|square| graphics::rectangle(RED, square, transform, gl));
        })
    }

    pub fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_front: SnakePieces =
        (*self.parts.front().expect("No front of snake found.")).clone();

        if (self.direction == Direction::UP && new_front.1 == 0)
            || (self.direction == Direction::LEFT && new_front.0 == 0)
           || (self.direction == Direction::DOWN && new_front.1 == rows - 1)
           || (self.direction == Direction::RIGHT && new_front.0 == cols - 1)
        {
            return false;
        }

        match self.direction {
            Direction::UP => new_front.1 -= 1,
            Direction::DOWN => new_front.1 += 1,
            Direction::LEFT => new_front.0 -= 1,
            Direction::RIGHT => new_front.0 += 1,
        }

        if !just_eaten {
            self.parts.pop_back();
        }

        if self.is_collide(new_front.0, new_front.1) {
            return false;
        }

        self.parts.push_front(new_front);
        true
    }

    pub fn is_collide(&self, x: u32, y:u32) -> bool {
        self.parts.iter().any(|p| x == p.0 && y == p.1)
    }
}
