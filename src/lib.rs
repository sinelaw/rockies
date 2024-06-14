mod utils;

use std::{
    cmp,
    fmt::{self, format},
};

use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V2 {
    x: i32,
    y: i32,
}

impl V2 {
    pub fn plus(&self, other: V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn cmul(&self, other: i32) -> V2 {
        V2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
    pub fn cdiv(&self, other: i32) -> V2 {
        V2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
    pub fn max(self, other: V2) -> V2 {
        V2 {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }
    pub fn min(self, other: V2) -> V2 {
        V2 {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }
}

const RESOLUTION: i32 = 100;
const MAX_VELOCITY: V2 = V2 {
    x: 1 * RESOLUTION,
    y: 1 * RESOLUTION,
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
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    pixels: Vec<u32>,
    gravity: V2,
    dt: i32,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        assert!(
            row <= self.width && column <= self.height,
            "out of bounds: {},{}",
            row,
            column
        );

        (column * self.height + row) as usize
    }
}

fn cells_to_pixels(cells: &Vec<Cell>) -> Vec<u32> {
    cells
        .iter()
        .map(|cell| match cell {
            Cell::Empty => 0xFFFFFF,
            Cell::Solid { color, inertia: _ } => color.to_u32(),
        })
        .collect()
}

#[wasm_bindgen]
impl Universe {
    fn clamp_position(&self, pos: V2, inertia: Inertia) -> (V2, Inertia) {
        let w = self.width as i32;
        let h = self.height as i32;
        let new_pos: V2 = pos
            .cmul(RESOLUTION)
            .plus(inertia.velocity.cmul(self.dt))
            .cdiv(RESOLUTION);
        let clamped_pos = V2 {
            x: match new_pos.x {
                x if x < 0 => -x,
                x if x >= w => w - (x - w) - 1,
                x => x,
            },
            y: match new_pos.y {
                y if y < 0 => -y,
                y if y >= h => h - (y - h) - 1,
                y => y,
            },
        };
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
        let clamped_inertia = Inertia {
            velocity: new_velocity,
            mass: inertia.mass,
        };
        (clamped_pos, clamped_inertia)
    }

    fn apply_forces(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
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

        for x in 0..self.width {
            for y in 0..self.height {
                let pos = V2 {
                    x: x as i32,
                    y: y as i32,
                };
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                match cell {
                    Cell::Empty => {}
                    Cell::Solid { color, inertia } => {
                        let (new_pos, new_inertia) = self.clamp_position(pos, inertia);
                        assert!(new_pos.x >= 0 && new_pos.y >= 0);
                        let new_idx = self.get_index(new_pos.x as u32, new_pos.y as u32);
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
        self.pixels = cells_to_pixels(&self.cells);

        //log!("{}", self.render());
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width: u32 = 64;
        let height: u32 = 64;
        let cells: Vec<Cell> = (0..width * height)
            .map(|i| {
                if i == 1000 {
                    Cell::Solid {
                        color: Color {
                            r: 0,
                            g: cmp::min(u8::MAX, i as u8),
                            b: 0,
                        },
                        inertia: Inertia {
                            velocity: V2 {
                                x: 0, //i as i32 % (2 * MAX_VELOCITY.x),
                                y: 0,
                            },
                            mass: 10,
                        },
                    }
                } else {
                    Cell::Empty
                }
            })
            .collect();
        let pixels = cells_to_pixels(&cells);
        Universe {
            width: width,
            height: height,
            cells: cells,
            pixels: pixels,
            gravity: V2 { x: 0, y: 1 },
            dt: 1,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const u32 {
        self.pixels.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        return Ok(());
    }
}
