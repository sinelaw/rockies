mod utils;
mod v2;

use std::fmt::{self};

use v2::V2;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        //       web_sys::console::log_1(&format!( $( $t )* ).into())
    };
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn to_u32(&self) -> u32 {
        (self.r as u32 * 256 * 256) + (self.g as u32 * 256) + self.b as u32
    }
}

const RESOLUTION: u32 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inertia {
    velocity: V2,
    force: V2,
    pos: V2,
    mass: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    color: Color,
    inertia: Inertia,
}

#[wasm_bindgen]
pub struct Universe {
    cells_width: u32,
    cells_height: u32,
    pixels_width: u32,
    pixels_height: u32,
    cells: Vec<Cell>,
    pixels: Vec<u32>,
    gravity: V2,
    dt: i32,
}

#[wasm_bindgen]
impl Universe {
    fn calc_forces(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.force = self.gravity.cmul(cell.inertia.mass);
        }
    }

    fn zero_forces(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.force = V2 { x: 0, y: 0 };
        }
    }

    fn update_pos(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(self.dt));
        }
    }

    fn update_vel(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.velocity = cell
                .inertia
                .velocity
                .plus(cell.inertia.force.cdiv(cell.inertia.mass))
                .cmul(self.dt);
        }
    }

    pub fn tick(&mut self) {
        self.calc_forces();
        self.update_vel();
        self.update_pos();
        self.zero_forces();

        self.render();

        //log!("{}", self.render());
    }

    fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let mut cells = Vec::new();

        let mut pixels = Vec::with_capacity((width * height) as usize);
        pixels.resize((width * height) as usize, 0xFFFFFF);
        let mut uni = Universe {
            cells_width: width * RESOLUTION,
            cells_height: height * RESOLUTION,
            pixels_width: width,
            pixels_height: height,
            cells: cells,
            pixels: pixels,
            gravity: V2 { x: 0, y: 10 },
            dt: 1,
        };

        let cell = Cell {
            color: Color { r: 0, g: 150, b: 0 },
            inertia: Inertia {
                velocity: V2 {
                    x: 1 * RESOLUTION as i32,
                    y: 0,
                },
                force: V2 { x: 0, y: 0 },
                pos: V2 { x: 5, y: 0 },
                mass: 1,
            },
        };
        uni.add_cell(cell);

        uni
    }

    fn render(&mut self) -> () {
        self.pixels.fill(0xFFFFFF);
        for cell in &self.cells {
            let x = cell.inertia.pos.x / (RESOLUTION as i32);
            let y = cell.inertia.pos.y / (RESOLUTION as i32);
            // out of the screen bounds
            if x < 0 || x >= self.pixels_width as i32 || y < 0 || y >= self.pixels_height as i32 {
                continue;
            }
            let pixel_idx = (y as u32 * self.pixels_width + x as u32) as usize;
            self.pixels[pixel_idx] = cell.color.to_u32()
        }
    }

    pub fn text_render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.pixels_width
    }

    pub fn height(&self) -> u32 {
        self.pixels_height
    }

    pub fn pixels(&self) -> *const u32 {
        self.pixels.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.pixels_width as usize) {
            for &pixel in line {
                let symbol = if pixel == 0xFFFFFF { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        for cell in self.cells.iter() {
            write!(f, "{cell:?}\n")?;
        }

        return Ok(());
    }
}
