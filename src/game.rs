extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::input::*;
use opengl_graphics::{GlGraphics};

use crate::food::{Food};
use crate::snake::{Snake};

#[derive(Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}


pub struct Game {
    pub gl: GlGraphics,
    pub rows: u32,
    pub cols: u32,
    pub snake: Snake,
    pub just_eaten: bool,
    pub square_width: u32,
    pub food: Food,
    pub score: u32,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl)
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width);

    }

    pub fn update(&mut self, _args: &UpdateArgs) -> bool {
        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);
        if self.just_eaten {
            use rand::Rng;
            use rand::thread_rng;

            let mut r = thread_rng();
            loop {
                let new_x = r.gen_range(0..self.cols);
                let new_y = r.gen_range(0..self.rows);

                if !self.snake.is_collide(new_x, new_y) {
                    self.food = Food {
                        x: new_x,
                        y: new_y
                    };
                    break;
                }
            }
        }
        true
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            &Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            _ => last_direction,
        }
    }
}