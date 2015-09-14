#![feature(append)]
#![feature(result_expect)]
#![feature(convert)]
#![cfg_attr(test, allow(dead_code, unused_imports, unused_variables))]
#![cfg_attr(tests, allow(dead_code, unused_imports, unused_variables))]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate num_cpus;
extern crate clap;
//use clap
//if normal then width, height
//if load then data file

mod save;

use graphics::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::input::*;
use piston::event_loop::*;
use std::thread;
use std::sync::mpsc::channel;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use clap::App;

const TITLE: &'static str = "Conway's Game of Life";
const TILE_SIZE: f64 = 4.0;
const UPDATE_TIME: f64 = 0.03;

fn main() {
    let width: usize; 
    let height: usize;
    let userseed: bool;

    let matches = App::new("conway-piston")
        .version("1.1")
        .author("Edward Yang <edward.yang6771@gmail.com>")
        .about("Conway's Game of Life implemented in Piston")
        .args_from_usage("-w --width=[WIDTH] 'optional - Sets a custom width for the default program.'
                         -h --height=[HEIGHT] 'optional - Sets a custom height for the default program.'
                         -s --seed=[SEED] 'optional - Sets the seed file to use (overrides width and height)'")
        .get_matches();
    if let Some(_) = matches.value_of("SEED") {
        userseed = true;
    } 
    else {
        userseed = false;
    }
    if let Some(w) = matches.value_of("WIDTH") {
        if !userseed {
            width = w.parse::<usize>().expect("Invalid width value.");
        }
        else {
            width = 200;
        } } else {
        width = 200;
    }
    if let Some(h) = matches.value_of("HEIGHT") {
        if !userseed {
            height = h.parse::<usize>().expect("Invalid height value.");
        } 
        else {
            height = 150;
        }
    } 
    else {
        height = 150;
    }

    let window_dimensions: [u32; 2] = [width as u32 * TILE_SIZE as u32, height as u32 * TILE_SIZE as u32];
    
    let opengl = OpenGL::V3_2;

    let window: GlutinWindow = 
        WindowSettings::new(TITLE, window_dimensions)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();
    
    let mut gfx = GlGraphics::new(opengl);

    let mut game = Game::new(width, height);
     
    if userseed {
        game.load_data(&save::parse(&matches.value_of("SEED").unwrap()));
    }
    else {
        game.randomize_values();
    }

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
            .map(|_| (0..width).map(|_| true).collect::<Vec<bool>>())
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
            let mut buffer_vals: Vec<(usize, Vec<Vec<bool>>)> = Vec::with_capacity(self.dimensions[1]);
            let mut startblock: usize = 0;
            let blocksize: usize = self.dimensions[1] as usize / max_threads;
            let mut endblock: usize = blocksize.clone();
            let (tx, rx) = channel();

            for tnum in 0usize..max_threads {
                let tx = tx.clone();
                let values: Vec<Vec<bool>> = self.values.clone();
                let dimensions: [usize; 2] = self.dimensions.clone();
                if tnum == max_threads - 1 {
                    endblock = self.dimensions[1];
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

            self.values.clear();
            for data in buffer_vals {
                self.values.append(&mut data.1.clone());
            }
            self.values.reserve(0);
        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::R => {self.values = self.seed.clone()},
            Key::G => {self.randomize_values()},
            Key::S => {save::save(&self.seed)},
            Key::C => {save::clear_saves()},
            _ => {}
        }
    }

    fn load_data(&mut self, data: &Vec<Vec<bool>>) {
        self.seed = data.to_owned();
        self.values = data.to_owned();
        self.seed.reserve(0);
        self.values.reserve(0);
    }

    fn get_neighbors(index: &(usize, usize), dimensions: &[usize; 2]) -> Vec<(usize, usize)> {
        let idx: (isize, isize) = (index.0 as isize, index.1 as isize);
        let collected: Vec<(isize, isize)> = vec![((idx.0 + 1), idx.1), ((idx.0 - 1), idx.1),
                                                     (idx.0, (idx.1 + 1)), (idx.0, (idx.1 - 1)),
                                                     ((idx.0 + 1), (idx.1 + 1)), ((idx.0 - 1), (idx.1 + 1)),
                                                     ((idx.0 + 1), (idx.1 - 1)), ((idx.0 - 1), (idx.1 - 1))];
        let mx: isize = dimensions[0] as isize - 1;
        let my: isize = dimensions[1] as isize - 1;
        let newcollected: Vec<(usize, usize)> = wrap_idxs(&collected, &mx, &my);
        newcollected
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
        self.seed.reserve(0);
        self.values.reserve(0);
    }
}

fn wrap_idx(idx: &(isize, isize), mx: &isize, my: &isize) -> (usize, usize) {
    let mut newidx = idx.clone();
    if &newidx.0 <= &0isize || &newidx.0 >= mx {
        newidx.0 = (&newidx.0 + mx) % mx;
    }
    if &newidx.1 <= &0isize || &newidx.1 >= my {
        newidx.1 = (&newidx.1 + my) % my;
    }
    (newidx.0 as usize, newidx.1 as usize)
}

fn wrap_idxs(idxs: &Vec<(isize, isize)>, mx: &isize, my: &isize) -> Vec<(usize, usize)> {
    idxs.iter()
        .map(|idx| wrap_idx(&idx, mx, my))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_wrapping() {
        assert_eq!(vec![(1, 1), (2, 149)], super::wrap_idxs(&vec![(1, 1), (2, -1)], &200, &150 ))
    }
}
