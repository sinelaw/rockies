mod utils;
mod v2;

use std::{collections::HashSet, fmt};

use v2::V2;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        //             web_sys::console::log_1(&format!( $( $t )* ).into())
        //      println!( $( $t )* );
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inertia {
    velocity: V2,
    force: V2,
    pos: V2,
    mass: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    color: Color,
    inertia: Inertia,
}

#[wasm_bindgen]
pub struct Universe {
    pixels_width: u32,
    pixels_height: u32,
    cells: Vec<Cell>,
    pixels: Vec<u32>,
    gravity: V2,
    dt: f64,
}

#[wasm_bindgen]
impl Universe {
    fn calc_forces(&mut self) {
        for cell in &mut self.cells {
            if cell.inertia.mass > 0 {
                cell.inertia.force = self.gravity.cmul(cell.inertia.mass as f64);
            }
        }
    }

    fn zero_forces(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.force = V2 { x: 0.0, y: 0.0 };
        }
    }

    fn update_pos(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(self.dt));
        }
    }

    fn update_vel(&mut self) {
        for cell in &mut self.cells {
            log!("cell: {cell:?}");
            if cell.inertia.mass > 0 {
                cell.inertia.velocity = cell
                    .inertia
                    .velocity
                    .plus(
                        cell.inertia
                            .force
                            .cdiv(cell.inertia.mass as f64)
                            .cmul(self.dt),
                    )
                    .min(V2 {
                        x: 0.5 / self.dt,
                        y: 0.5 / self.dt,
                    })
            }
        }
    }

    fn collect_collisions(&mut self) -> HashSet<(usize, usize)> {
        let mut collisions: HashSet<(usize, usize)> = HashSet::new();
        for (cell1_idx, cell1) in self.cells.iter().enumerate() {
            for (cell2_idx, cell2) in self.cells.iter().enumerate() {
                if cell1_idx == cell2_idx {
                    continue;
                }
                // collision between infinite masses?!
                if (cell1.inertia.mass == 0) && (cell2.inertia.mass == 0) {
                    continue;
                }

                let norm = cell1.inertia.pos.minus(cell2.inertia.pos);
                if norm.magnitude() > 1.0 {
                    continue;
                }

                let rel_velocity = cell1.inertia.velocity.minus(cell2.inertia.velocity);

                // if the dot product is negative, the two objects are colliding,
                let dot = rel_velocity.dot(norm);
                if dot > 0.0 {
                    // moving away from each other
                    continue;
                }
                if dot * dot < 0.0001 {
                    // negligible velocity (floating point error)
                    continue;
                }
                let key = (cell1_idx.min(cell2_idx), cell1_idx.max(cell2_idx));

                collisions.insert(key);

                log!("collision: {key:?} {norm:?} {dot:?}");
                log!("cell1: {cell1:?}");
                log!("cell2: {cell2:?}");

                // println!("collisions: {:?}", collisions);
            }
        }
        collisions
    }

    fn calc_collisions(&mut self) {
        let collisions = self.collect_collisions();
        for (cell1_idx, cell2_idx) in collisions {
            let cell2 = self.cells[cell2_idx];
            let cell1 = self.cells[cell1_idx];

            let rel_velocity = cell1.inertia.velocity.minus(cell2.inertia.velocity);
            let norm = cell1.inertia.pos.minus(cell2.inertia.pos);
            // coefficient of restitution
            let e = 0.99;
            let collision_vel: f64 = rel_velocity.dot(norm) as f64 * -(1.0 + e);

            // collision between infinite masses?!
            if (cell1.inertia.mass == 0) && (cell2.inertia.mass == 0) {
                continue;
            }
            if cell1.inertia.mass == 0 {
                // cell1 = infinite mass, cell2 = finite mass
                let cell = &mut self.cells[cell2_idx];
                cell.inertia.velocity = cell2.inertia.velocity.plus(norm.cmul(collision_vel));
            }
            if cell2.inertia.mass == 0 {
                // cell1 = finite mass, cell2 = infinite mass
                let cell = &mut self.cells[cell1_idx];
                cell.inertia.velocity = cell1.inertia.velocity.plus(norm.cmul(collision_vel));
            }
            if (cell1.inertia.mass != 0) && (cell2.inertia.mass != 0) {
                // both finite masses
                let im1 = 1.0 / (cell1.inertia.mass as f64);
                let im2 = 1.0 / (cell2.inertia.mass as f64);
                let impulse = collision_vel / (im1 + im2);

                {
                    let cell = &mut self.cells[cell1_idx];
                    cell.inertia.velocity = cell1.inertia.velocity.plus(norm.cmul(impulse * im1));
                }
                {
                    let cell = &mut self.cells[cell2_idx];
                    cell.inertia.velocity = cell2.inertia.velocity.minus(norm.cmul(impulse * im2));
                }
            }
        }
    }

    pub fn tick(&mut self) {
        for _ in 0..((1.0 / self.dt) as usize) {
            self.calc_forces();
            self.update_vel();

            self.calc_collisions();
            self.update_pos();
            self.zero_forces();
        }

        self.render();

        //log!("{}", self.render());
    }

    fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    fn wall_cell(&self, x: f64, y: f64) -> Cell {
        Cell {
            color: Color { r: 150, g: 0, b: 0 },
            inertia: Inertia {
                velocity: V2 { x: 0.0, y: 0.0 },
                force: V2 { x: 0.0, y: 0.0 },
                pos: V2 { x, y },
                mass: 0,
            },
        }
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let mut uni = Universe {
            pixels_width: width,
            pixels_height: height,
            cells: Vec::new(),
            pixels: {
                let mut pixels = Vec::with_capacity((width * height) as usize);
                pixels.resize((width * height) as usize, 0xFFFFFF);
                pixels
            },
            gravity: V2 { x: 0.0, y: 1.0 },
            dt: 0.01,
        };

        for i in 0..1 {
            uni.add_cell(Cell {
                color: Color {
                    r: (10 * i) % 255,
                    g: 150,
                    b: (155 * i) % 255,
                },
                inertia: Inertia {
                    velocity: V2 { x: 5.0, y: 0.0 },
                    force: V2 { x: 0.0, y: 0.0 },
                    pos: V2 {
                        x: i as f64 * 5.0,
                        y: 10.0,
                    },
                    mass: 1,
                },
            });
        }
        for x in 0..width {
            uni.add_cell(uni.wall_cell(x as f64, 0.0));
            uni.add_cell(uni.wall_cell(x as f64, (height - 1) as f64));
        }
        for y in 0..height {
            uni.add_cell(uni.wall_cell(0.0, y as f64));
            uni.add_cell(uni.wall_cell((width - 1) as f64, y as f64));
        }

        uni
    }

    fn render(&mut self) -> () {
        self.pixels.fill(0xFFFFFF);
        for cell in &self.cells {
            let x = cell.inertia.pos.x;
            let y = cell.inertia.pos.y;
            // out of the screen bounds
            if x < 0.0 || x >= self.pixels_width as f64 || y < 0.0 || y >= self.pixels_height as f64
            {
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

        return Ok(());
    }
}
