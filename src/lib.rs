use std::fmt;

mod grid;
mod int_pair_set;
mod universe;
mod utils;
mod v2;
use universe::{round, Cell, CellIndex, Color, Inertia, Stats, Universe};

use v2::V2;
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
                let neighbors_count = self.universe.grid.get(x as usize, y as usize).0;
                self.pixels[(y * self.width + x) as usize] -= (0x20 * neighbors_count) as u32;
            }
        }
        for cell in &self.universe.cells {
            let x = round(cell.inertia.pos.x) as i32;
            let y = round(cell.inertia.pos.y) as i32;

            // out of the screen bounds
            if !self.is_in_bounds(x, y) {
                continue;
            }
            let pixel_idx = (y * (self.width as i32) + x) as usize;
            self.pixels[pixel_idx] = if cell.collisions > 0 {
                0xFF0000
            } else {
                cell.color.to_u32()
            }
        }
    }

    pub fn text_render(&self) -> String {
        self.to_string()
    }

    pub fn click(&mut self, x: i32, y: i32) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        // unstick some cells
        self.universe.unstick_cells(x as usize, y as usize);

        // add a new cell
        let r = (x % 17) as f64 / 17.0 - 1.0;
        self.universe.add_cell(Cell {
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
            },
            collisions: 0,
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
