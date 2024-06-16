mod utils;
mod v2;

use std::{
    cmp,
    fmt::{self},
};

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

const RESOLUTION: u32 = 2;
const MAX_VELOCITY: V2 = V2 {
    x: 10 * RESOLUTION as i32,
    y: 10 * RESOLUTION as i32,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inertia {
    velocity: V2,
    mass: u8,
}

impl Inertia {
    pub fn accelerate(&self, accel: V2) -> Inertia {
        Inertia {
            velocity: self.velocity.plus(accel).min(MAX_VELOCITY),
            mass: self.mass,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Solid { color: Color, inertia: Inertia },
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

impl Universe {
    fn cell_index(&self, x: u32, y: u32) -> usize {
        (y * self.cells_width + x) as usize
    }
}

#[wasm_bindgen]
impl Universe {
    fn render(&mut self) -> () {
        self.pixels.fill(0xFFFFFF);
        for cell_y in 0..self.cells_height {
            for cell_x in 0..self.cells_width {
                let idx = self.cell_index(cell_x, cell_y);
                let cell = self.cells[idx];

                match cell {
                    Cell::Empty => (),
                    Cell::Solid { color, inertia: _ } => {
                        let x = cell_x / (RESOLUTION as u32);
                        let y = cell_y / (RESOLUTION as u32);
                        let pixel_idx = (y * self.pixels_width + x) as usize;
                        self.pixels[pixel_idx] = color.to_u32()
                    }
                };
            }
        }
    }

    fn clamp_position(&self, pos: V2, inertia: Inertia) -> (V2, Inertia) {
        let w = self.cells_width as i32;
        let h = self.cells_height as i32;
        let new_pos: V2 = pos
            .cmul(RESOLUTION as i32)
            .plus(inertia.velocity.cmul(self.dt))
            .cdiv(RESOLUTION as i32);

        let clamped_pos = V2 {
            x: match new_pos.x {
                x if x < 0 || x >= w => pos.x,
                x => x,
            },
            y: match new_pos.y {
                y if y < 0 || y >= h => pos.y,
                y => y,
            },
        };
        // When an object reaches the immovable wall it reverses
        // the direction inside the time step (e.g. halfway through),
        // so simply reversing the velocity is wrong. In the first part of the time window,
        // the object is still moving in the same direction, and in the second part it's reversed.
        // 1. v + a*dt1
        // 2. -(v + a*dt1) + a*dt2 = -v + a*(dt2 - dt1)
        // In the most extreme cases, final velocity is either: -v - a*dt or -v + a*dt
        // If we just ignore the acceleration instead we get an inaccurate result, but it conserves energy.
        let new_velocity = V2 {
            x: match new_pos.x {
                x if x < 0 || x >= w => -inertia.velocity.x,

                _ => inertia.velocity.x,
            },

            y: match new_pos.y {
                y if y < 0 || y >= h => -inertia.velocity.y,
                _ => inertia.velocity.y,
            },
        };
        let corrected_velocity = match new_velocity {
            v if v == inertia.velocity => v,
            _ => new_velocity.minus(self.gravity.cmul(self.dt)),
        };
        let clamped_inertia = Inertia {
            velocity: corrected_velocity,
            mass: inertia.mass,
        };
        (clamped_pos, clamped_inertia)
    }

    fn apply_forces(&mut self) {
        let mut next = self.cells.clone();

        for cell_y in 0..self.cells_height {
            for cell_x in 0..self.cells_width {
                let idx = self.cell_index(cell_x, cell_y);
                let cell = self.cells[idx];
                let next_cell = match cell {
                    Cell::Empty => cell,

                    Cell::Solid { color, inertia } => Cell::Solid {
                        color,
                        inertia: {
                            log!("{inertia:?}, pos: {row},{col}");
                            inertia.accelerate(self.gravity.cmul(self.dt))
                        },
                    },
                };

                /*  match next_cell {
                    Cell::Empty => {}
                    c @ Cell::Solid { .. } => log!("Cell[{row},{col}] = {c:?}"),
                } */
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn update_positions(&mut self) {
        let mut next = self.cells.clone();

        for x in 0..self.cells_width {
            for y in 0..self.cells_height {
                let pos = V2 {
                    x: x as i32,
                    y: y as i32,
                };
                let idx = self.cell_index(x, y);
                let cell = self.cells[idx];
                match cell {
                    Cell::Empty => {}
                    Cell::Solid { color, inertia } => {
                        let (new_pos, new_inertia) = self.clamp_position(pos, inertia);
                        assert!(new_pos.x >= 0 && new_pos.y >= 0);
                        let new_idx = self.cell_index(new_pos.x as u32, new_pos.y as u32);
                        next[idx] = self.cells[new_idx];
                        next[new_idx] = Cell::Solid {
                            color: color,
                            inertia: new_inertia,
                        }
                    }
                }
            }
        }
        self.cells = next;
    }

    pub fn tick(&mut self) {
        self.apply_forces();
        self.update_positions();
        self.render();

        //log!("{}", self.render());
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let cells: Vec<Cell> = (0..(width * height * (RESOLUTION * RESOLUTION) as u32))
            .map(|i| {
                if i % 2000 == 0 {
                    Cell::Solid {
                        color: Color {
                            r: 0,
                            g: cmp::min(u8::MAX, i as u8),
                            b: 0,
                        },
                        inertia: Inertia {
                            velocity: V2 { x: 5, y: 0 },
                            mass: 10,
                        },
                    }
                } else {
                    Cell::Empty
                }
            })
            .collect();

        let mut pixels = Vec::with_capacity((width * height) as usize);
        pixels.resize((width * height) as usize, 0xFFFFFF);
        Universe {
            cells_width: width * RESOLUTION,
            cells_height: height * RESOLUTION,
            pixels_width: width,
            pixels_height: height,
            cells: cells,
            pixels: pixels,
            gravity: V2 { x: 0, y: 1 },
            dt: 1,
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
            if cell == &Cell::Empty {
                continue;
            }
            write!(f, "{cell:?}\n")?;
        }

        return Ok(());
    }
}
