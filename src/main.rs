//MOVE SQUARE
#![feature(globs)] //can use foo::*;

extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use std::cmp::{max, min}; //use for edge behav

use opengl_graphics::{
    Gl,
};
use sdl2_game_window::WindowSDL2;
use graphics::*;
use piston::{
    EventIterator,
    EventSettings,
    WindowSettings,
    Input,
    Render,
    Update
};

use piston::input::keyboard::{
    Up, Down, Left, Right,
    W, J

};

use piston::input::{
    Keyboard,
    Press,
    Release
};

//for random jitter
use std::rand;
use std::rand::Rng;

pub static GRID_HEIGHT: int = 5;
pub static GRID_WIDTH: int = 5;

pub static BLOCK_SIZE: int = 100;

pub static WINDOW_HEIGHT: int = GRID_HEIGHT * BLOCK_SIZE;
pub static WINDOW_WIDTH: int = GRID_WIDTH * BLOCK_SIZE;

enum Direction {
    UpDir,
    DownDir,
    LeftDir,
    RightDir,
    Stop
}

struct GameState {
    pub x: int, pub y: int,
    pub max_x: int, pub max_y: int,

    pub edge_behav: bool, //false-stop, true-wrap
    pub jitter_behav: bool, //true-jitters
    pub next_mov: Direction //direction of movement in the next tick. Stop means no mov
}

impl GameState {
    pub fn new(x: int, y: int, max_x: int, max_y: int, edge_behav: bool, jitter_behav: bool) -> GameState {
        GameState {
            x: x,
            y: y,
            max_x: max_x,
            max_y: max_y,
            edge_behav: edge_behav,
            jitter_behav: jitter_behav,
            next_mov: Stop
        }
    }

    pub fn mov(&mut self, x: int, y: int) {
        match self.edge_behav {
            //stopping behavior. `self.max_x - 1` because range is (0, len-1)
            false => { self.x = min(max(self.x + x, 0), self.max_x - 1);
                       self.y = min(max(self.y + y, 0), self.max_y - 1);
            },
            //wrapping behavior
            true => {
                self.x += x;
                if self.x > self.max_x - 1 {self.x = 0}
                else if self.x < 0 {self.x = self.max_x - 1};
                self.y += y;
                if self.y > self.max_y - 1 {self.y = 0}
                else if self.y < 0 {self.y = self.max_x - 1};
            }
        }
    }
    // flip stopping/wrapping behavior
    pub fn change_edge_behav(&mut self) {self.edge_behav = !self.edge_behav}
    //start/stop jittering
    pub fn change_jitter_behav(&mut self) {self.jitter_behav = !self.jitter_behav}

    pub fn jitter(&mut self) {
        if self.jitter_behav {
            let mut rng = rand::task_rng();
            let r = rng.gen::<uint>() % 4; // %4 trick to get range 0-3
            match r {
                0 => {self.mov(1, 0)},
                1 => {self.mov(-1, 0)},
                2 => {self.mov(0, 1)},
                3 => {self.mov(0, -1)},
                _ => {}
            }
        }
    }
}

fn main() {
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        WindowSettings {
            title: "moving square".to_string(),
            size: [WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0
        }
    );

    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };

    let ref mut gl = Gl::new();

    let mut game = GameState::new(GRID_WIDTH/2, GRID_HEIGHT/2, GRID_WIDTH, GRID_HEIGHT, false, false);

    let mut jitter_counter: uint = 11;
    let mut slide_counter: uint = 11;

    for event in EventIterator::new(&mut window, &event_settings) {
        match event {
            Render(args) => {
                gl.viewport(0, 0, args.width as i32, args.height as i32);
                let c = Context::abs(args.width as f64, args.height as f64);
                c.rgb(1.0, 1.0, 1.0).draw(gl);
                c.square(
                        (game.x * BLOCK_SIZE) as f64,
                        (game.y * BLOCK_SIZE) as f64,
                        BLOCK_SIZE as f64
                    )
                    .margin(10.0)
                    .border_radius(10.0)
                    .rgb(1.0, 0.0, 0.0)
                    .draw(gl);
            },

            Input(Press(Keyboard(key))) => {
                match key {
                    Up => {game.next_mov = UpDir},
                    Down => {game.next_mov = DownDir},
                    Left => {game.next_mov = LeftDir},
                    Right => {game.next_mov = RightDir},
                    W => {game.change_edge_behav()},
                    J => {game.change_jitter_behav()},
                    _ => {}
                }
            }

            Input(Release(Keyboard(key))) => {
                game.next_mov = match key {
                      Up | Down | Left | Right => Stop,
                      _ => game.next_mov
                }
            }

            Update(_) => {
                jitter_counter += 1;
                if jitter_counter == 12 {jitter_counter = 0; game.jitter()};

                slide_counter += 1;
                if slide_counter == 12 {
                    slide_counter = 0;
                    match game.next_mov {
                        UpDir => {game.mov(0, -1)},
                        DownDir => {game.mov(0, 1)},
                        LeftDir => {game.mov(-1, 0)},
                        RightDir => {game.mov(1,0)},
                        _ => {}
                    }
                }
            }
            _ => {}

        }
    }
}
