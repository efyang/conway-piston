#![feature(append)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate num_cpus;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::*;
use piston::event_loop::*;
use std::thread;
use std::sync::mpsc::channel;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;

const TITLE: &'static str = "Conway's Game of Life";
const BOARD_WIDTH: usize = 200;
const BOARD_HEIGHT: usize = 150;
const TILE_SIZE: f64 = 4.5;
const UPDATE_TIME: f64 = 0.03;

fn main() {

    let window_dimensions: [u32; 2] = [BOARD_WIDTH as u32 * TILE_SIZE as u32, BOARD_HEIGHT as u32 * TILE_SIZE as u32];
    
    let opengl = OpenGL::V3_2;

    let window: GlutinWindow = 
        WindowSettings::new(TITLE, window_dimensions)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    
    let mut gfx = GlGraphics::new(opengl);

    let mut game = Game::new(BOARD_WIDTH, BOARD_HEIGHT);

    let max_threads: usize = num_cpus::get();
    
    for e in window.events() {
        use piston::input::Button;
        if let Some(args) = e.render_args() {
            let t = Context::new_viewport(args.viewport()).transform;
            game.render(t, &mut gfx);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_press(key);
        }

        if let Some(args) = e.update_args() {
            game.update(args.dt, max_threads);
        } 
    }
}

struct Game {
    seed: Vec<Vec<bool>>,
    values: Vec<Vec<bool>>,
    dimensions: [usize; 2],
    time: f64,
    update_time: f64,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        let newseed: Vec<Vec<bool>> = (0..height)
            .map(|_| (0..width).map(|_| rand::random()).collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();
        Game {
            seed: newseed.clone(),
            values: newseed.clone(),
            dimensions: [width, height],
            time: UPDATE_TIME,
            update_time: UPDATE_TIME,
        }
    }

    fn render(&mut self, t: math::Matrix2d, gfx: &mut GlGraphics) {
        for y in 0..self.dimensions[1] {
            for x in 0..self.dimensions[0] {
                let status: types::Color;
                if self.values[y][x] {
                    status = color::BLACK;//alive
                }
                else {
                    status = color::WHITE;
                }
                rectangle(status,
                          rectangle::square(x as f64 * TILE_SIZE, y as f64 * TILE_SIZE, TILE_SIZE),
                          t,
                          gfx)
            }
        }
    }

    fn update(&mut self, dt: f64, max_threads: usize) {
        self.time += dt;

        if self.time > self.update_time {
            self.time -= self.update_time;
            //check alive and update
            let mut buffer_vals: Vec<(usize, Vec<Vec<bool>>)> = Vec::new();
            
            let mut startblock: usize = 0;
            let blocksize: usize = BOARD_HEIGHT as usize / max_threads;
            let mut endblock: usize = blocksize.clone();

            //for y in 0..self.dimensions[1] {
                //for x in 0..self.dimensions[0] {
                    //buffer_vals[x][y] = self.is_alive(&(x, y));
                //}
            //}
            let (tx, rx) = channel();

            for tnum in 0usize..max_threads {
                let tx = tx.clone();
                //let startblock = startblock.clone();
                //let endblock = endblock.clone();
                let values: Vec<Vec<bool>> = self.values.clone();
                let dimensions: [usize; 2] = self.dimensions.clone();
                if tnum == max_threads - 1 {
                    endblock = BOARD_HEIGHT;
                }
                thread::spawn(move || {
                    let mut data: Vec<Vec<bool>> = Vec::new();
                    for y in startblock..endblock {
                        data.push((0..dimensions[0])
                                  .map(|x| Game::is_alive(&values, &(x, y), &dimensions))
                                  .collect::<Vec<bool>>()); 
                    }
                    tx.send((tnum, data)).unwrap();
                });
                startblock += blocksize;
                endblock += blocksize;
            }
            for _ in 0..max_threads {
                buffer_vals.push(rx.recv().unwrap());
            }
            buffer_vals.sort_by(|a, b| (a.0).cmp(&b.0));

            let mut valbuf: Vec<Vec<bool>> = Vec::new();
            //self.values.clear();
            for data in buffer_vals {
                valbuf.append(&mut data.1.clone());
            }
            self.values = valbuf.clone();

        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::R => {self.values = self.seed.clone()},
            Key::G => {self.randomize_values()},
            _ => {}
        }
    }

    fn get_neighbors(idx: &(usize, usize), dimensions: &[usize; 2]) -> Vec<(usize, usize)> {
        let mut collected = vec![((idx.0 + 1), idx.1), ((idx.0 - 1), idx.1),
                                 (idx.0, (idx.1 + 1)), (idx.0, (idx.1 - 1)),
                                 ((idx.0 + 1), (idx.1 + 1)), ((idx.0 - 1), (idx.1 + 1)),
                                 ((idx.0 + 1), (idx.1 - 1)), ((idx.0 - 1), (idx.1 - 1))];
        if idx.0 == 0 || idx.0 >= dimensions[0] - 1 {
            collected = collected.iter().map(|x| ((x.0 % dimensions[0]), x.1)).collect();
        }
        if idx.1 == 0 || idx.1 >= dimensions[1] - 1 {
            collected = collected.iter().map(|x| (x.0, (x.1 % dimensions[1]))).collect();
        }
        collected
    }

    fn is_alive(values: &Vec<Vec<bool>>, idx: &(usize, usize), dimensions: &[usize; 2]) -> bool {
        let neighbors = Game::get_neighbors(idx, &dimensions);
        let statuses: Vec<bool> = neighbors.iter()
            .map(|i| values[i.1][i.0])
            .collect();
        let live: usize = statuses.iter().fold(0usize, |acc, &item| if item { acc + 1 } else {acc});
        if values[idx.1][idx.0] {
            //if cell is already alive 
            if live < 2 || live > 3 {
                return false;
            }
            return true; 
        }
        else {
            //if cell is dead
            if live == 3 {
                return true;
            }
            return false;
        }
    }

    fn randomize_values(&mut self) {
        let newseed: Vec<Vec<bool>> = (0..self.dimensions[1])
            .map(|_| (0..self.dimensions[0]).map(|_| rand::random()).collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>(); 
        self.seed = newseed.clone();
        self.values = newseed.clone();
    }
}
