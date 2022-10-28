extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;

struct Game {
    gl: GlGraphics,
    rows: u32,
    cols: u32,
    snake: Snake,
    just_eaten: bool,
    square_width: u32,
    food: Food,
    score: u32,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl)
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width);

    }

    fn update(&mut self, _args: &UpdateArgs) -> bool {
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

    fn pressed(&mut self, btn: &Button) {
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



#[derive(Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Snake {
    gl: GlGraphics,
    width: u32,
    direction: Direction,
    parts: LinkedList<SnakePieces>,
}

#[derive(Clone)]
pub struct SnakePieces(u32, u32);

impl Snake {
    fn render(&mut self, args: &RenderArgs) {
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

    fn is_collide(&self, x: u32, y:u32) -> bool {
        self.parts.iter().any(|p| x == p.0 && y == p.1)
    }
}

pub struct Food {
    x: u32,
    y: u32,
}

impl Food {
    fn update(&mut self, snake: &Snake) -> bool {
        let front = snake.parts.front().unwrap();

        if front.0 == self.x && front.1 == self.y {
            true
        } else {
            false
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {

        const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLACK, square, transform, gl);
        })
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let with = COLS * SQUARE_WIDTH;
    let height = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [with,height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        food: Food { x: 1, y: 1 },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            width: SQUARE_WIDTH,
            direction: Direction::DOWN,
            parts: LinkedList::from_iter((vec![SnakePieces(COLS/2, ROWS/2)]).into_iter())
        }
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
          game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.score)
}
