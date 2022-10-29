extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

mod food;
use food::Food;

mod snake;
use snake::{SnakePieces, Snake};

mod game;
use game::{Game, Direction};

use std::collections::LinkedList;

fn bootstrap_game(rows: u32, cols: u32, square_width: u32, opengl: OpenGL ) -> Game {
    use rand::Rng;
    let x = rand::thread_rng().gen_range(1..cols);
    let y  = rand::thread_rng().gen_range(1..rows);

    return Game {
        gl: GlGraphics::new(opengl),
        rows: rows,
        cols: cols,
        square_width: square_width,
        just_eaten: false,
        food: Food { 
            x:  x, 
            y: y, 
        },
        score: 0,
        snake: Snake {
            gl: GlGraphics::new(opengl),
            width: square_width,
            direction: Direction::DOWN,
            parts: LinkedList::from_iter((vec![SnakePieces(cols/2, rows/2)]).into_iter())
        }
    };
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

    let mut game = bootstrap_game(ROWS, COLS, SQUARE_WIDTH, opengl);
  
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
          game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                game = bootstrap_game(ROWS, COLS, SQUARE_WIDTH, opengl);
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
