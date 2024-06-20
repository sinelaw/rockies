use std::fmt;
mod inertia;

mod assets;
mod color;
mod grid;
mod universe;
mod utils;
mod v2;
use color::Color;

use inertia::Inertia;
use universe::{Cell, CellIndex, Stats, Universe};

use v2::{V2i, V2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    universe: Universe,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        utils::set_panic_hook();
        console_error_panic_hook::set_once();
        Self {
            width,
            height,
            pixels: vec![0xFFFFFF; (width * height) as usize],
            universe: Universe::new(width, height),
        }
    }

    pub fn pixels(&self) -> *const u32 {
        self.pixels.as_ptr()
    }

    pub fn tick(&mut self) {
        self.render();
        self.universe.tick();
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < (self.width as i32) && y >= 0 && y < (self.height as i32)
    }

    pub fn render(&mut self) -> () {
        self.pixels.fill(0xFFFFFF);

        for x in 0..self.width {
            for y in 0..self.height {
                let pos = V2i::new(x as i32, y as i32);
                let get_res = self.universe.cells.get(V2i::new(x as i32, y as i32));
                let pixel_idx = (pos.y * (self.width as i32) + pos.x) as usize;
                match get_res {
                    Some(cell) => {
                        self.pixels[pixel_idx] = if cell.inertia.collision_stats > 0 {
                            0xFF0000
                        } else {
                            cell.color.to_u32()
                        }
                    }
                    None => (),
                }
            }
        }
        self.universe
            .player
            .render(&mut self.pixels, self.width, self.height);
    }

    pub fn text_render(&self) -> String {
        self.to_string()
    }

    pub fn key(&mut self, key: char) {
        match key {
            'a' => self.universe.player.move_left(),
            'd' => self.universe.player.move_right(),
            'w' => self.universe.player.move_up(),
            's' => self.universe.player.move_down(),
            ' ' => {
                self.universe.player.next_frame();
                let pos: V2i = self.universe.player.inertia.pos.round();
                self.universe.cells.add_cell(Cell {
                    index: CellIndex { index: 0 },
                    color: Color::from_hsv(
                        ((self.universe.player.frame / 10) % 360) as f64,
                        1.0,
                        1.0,
                    ),
                    inertia: Inertia {
                        velocity: V2::new(1.0 * (self.universe.player.direction as f64), -1.0),
                        force: V2::zero(),
                        pos: self.universe.player.mouth_pos(),
                        mass: 1,
                        elasticity: 0.5,
                        collision_stats: 0,
                    },
                });
            }
            'k' => {
                let pos: V2i = self.universe.player.inertia.pos.round();
                for x in 0..self.universe.player.w {
                    self.universe.cells.remove_cells(
                        (pos.x + x as i32) as usize,
                        (pos.y) as usize,
                        self.universe.player.h - 1,
                    );
                }
            }
            _ => (),
        }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        // unstick some cells
        self.universe.cells.unstick_cells(x as usize, y as usize, 3);

        // add a new cell
        let r = (x % 17) as f64 / 17.0 - 1.0;
        self.universe.cells.add_cell(Cell {
            index: CellIndex { index: 0 },
            color: Color {
                r: 0,
                g: 150,
                b: ((155 * y) % 255) as u8,
            },
            inertia: Inertia {
                velocity: V2::zero(),
                force: V2::zero(),
                pos: V2 {
                    x: x as f64 + r,
                    y: y as f64 + r,
                },
                mass: 1,
                elasticity: 0.5,
                collision_stats: 0,
            },
        });
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn stats(&mut self) -> Stats {
        self.universe.stats()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width as usize) {
            for &pixel in line {
                let symbol = if pixel == 0xFFFFFF { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        return Ok(());
    }
}
