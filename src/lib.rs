mod grid;
mod smallint_set;
mod utils;
mod v2;

use std::fmt;

use grid::Grid;
use smallint_set::IntPairSet;
use v2::V2;
use wasm_bindgen::prelude::*;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        //             web_sys::console::log_1(&format!( $( $t )* ).into())
        //   println!( $( $t )* );
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
    index: usize,
    color: Color,
    inertia: Inertia,
}

#[wasm_bindgen]
pub struct Universe {
    pixels_width: u32,
    pixels_height: u32,
    cells: Vec<Cell>,

    grid: Grid<usize>,
    pixels: Vec<u32>,
    gravity: V2,
    dt: f64,
}

fn inverse_mass(cell: Cell) -> f64 {
    if cell.inertia.mass == 0 {
        return 0.0;
    }
    return 1.0 / (cell.inertia.mass as f64);
}

fn round(x: f64) -> i32 {
    (x + 0.5) as i32
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
        for cell in &self.cells {
            if self.is_in_bounds(cell.inertia.pos) {
                self.grid
                    .clear(cell.inertia.pos.x as usize, cell.inertia.pos.y as usize);
            }
        }

        for cell in &mut self.cells {
            cell.inertia.pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(self.dt));
        }
        for cell in &self.cells {
            if self.is_in_bounds(cell.inertia.pos) {
                // store in the grid
                self.grid.put(
                    cell.inertia.pos.x as usize,
                    cell.inertia.pos.y as usize,
                    cell.index,
                );
            }
        }
    }

    fn log_cells(&self) {
        for cell in &self.cells {
            if cell.inertia.mass == 0 {
                continue;
            }
            log!("cell: {cell:?}");
        }
    }

    fn update_vel(&mut self) {
        for cell in &mut self.cells {
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

    fn is_in_bounds(&self, pos: V2) -> bool {
        pos.x >= 0.0
            && pos.x < self.pixels_width as f64
            && pos.y >= 0.0
            && pos.y < self.pixels_height as f64
    }

    fn collect_collisions(&mut self) -> Vec<(usize, usize)> {
        let mut collisions_list: Vec<(usize, usize)> = Vec::new();
        let mut collisions_map = IntPairSet::new(self.cells.len());

        for (cell1_idx, cell1) in self.cells.iter().enumerate() {
            if !self.is_in_bounds(cell1.inertia.pos) {
                continue;
            }
            let neighbors = self
                .grid
                .get(cell1.inertia.pos.x as usize, cell1.inertia.pos.y as usize);
            for ns in neighbors {
                for ns2 in ns {
                    for cell2_idx in ns2 {
                        if cell1_idx == *cell2_idx {
                            continue;
                        }

                        if collisions_map.contains(cell1_idx, *cell2_idx) {
                            continue;
                        }

                        collisions_map.put(cell1_idx, *cell2_idx);

                        let cell2 = &self.cells[*cell2_idx];
                        // collision between infinite masses?!
                        if (cell1.inertia.mass == 0) && (cell2.inertia.mass == 0) {
                            continue;
                        }

                        let normal = cell1.inertia.pos.minus(cell2.inertia.pos);
                        let radius = 1.0; // they're actually boxes but ok
                        if normal.magnitude_sqr() > radius * radius {
                            continue;
                        }

                        let rel_velocity = cell1.inertia.velocity.minus(cell2.inertia.velocity);

                        // skip objects that have negligible relative velocity
                        if rel_velocity.magnitude_sqr() < 0.001 {
                            continue;
                        }

                        // if the dot product is negative, the two objects are colliding,
                        let dot = rel_velocity.dot(normal);
                        if dot > 0.0 {
                            // moving away from each other
                            continue;
                        }
                        if dot * dot < 0.0001 {
                            // negligible velocity (floating point error)
                            continue;
                        }

                        collisions_list.push((cell1_idx, *cell2_idx));

                        log!("collision: {key:?} {normal:?} {dot:?}");
                        log!("cell1: {cell1:?}");
                        log!("cell2: {cell2:?}");

                        // println!("collisions: {:?}", collisions);
                    }
                }
            }
        }
        collisions_list
    }

    fn calc_collisions(&mut self) {
        let collisions = self.collect_collisions();
        for (cell1_idx, cell2_idx) in collisions {
            let cell2 = self.cells[cell2_idx];
            let cell1 = self.cells[cell1_idx];

            // collision between infinite masses?!
            if (cell1.inertia.mass == 0) && (cell2.inertia.mass == 0) {
                continue;
            }

            let rel_velocity = cell1.inertia.velocity.minus(cell2.inertia.velocity);
            let normal = cell1.inertia.pos.minus(cell2.inertia.pos);
            // coefficient of restitution
            let e = 0.99;
            let collision_vel: f64 = rel_velocity.dot(normal) as f64 * -(1.0 + e);

            // for simplicity the rest here treats them as circles, not boxes:
            let distance = normal.magnitude();

            let normal_direction = if distance == 0.0 {
                // the two are perfectly aligned on top of each other
                V2 { x: 1.0, y: 0.0 }
            } else {
                normal.cdiv(distance)
            };

            let im1 = inverse_mass(cell1);
            let im2 = inverse_mass(cell2);

            let penetration = 1.0 - distance; // 1.0 = "radius"
            let slop = 0.02;
            let pos_correct = normal_direction
                .cmul((penetration - slop) / (im1 + im2))
                .cmul(0.2);

            let impulse = collision_vel / (im1 + im2);

            {
                let cell = &mut self.cells[cell1_idx];
                cell.inertia.velocity = cell1
                    .inertia
                    .velocity
                    .plus(normal_direction.cmul(impulse * im1));
                cell.inertia.pos = cell1.inertia.pos.plus(pos_correct.cmul(im1));
            }
            {
                let cell = &mut self.cells[cell2_idx];
                cell.inertia.velocity = cell2
                    .inertia
                    .velocity
                    .minus(normal_direction.cmul(impulse * im2));
                cell.inertia.pos = cell2.inertia.pos.minus(pos_correct.cmul(im2));
            }

            log!("rel_velocity: {rel_velocity:?}");
            log!("norm: {normal:?}");
            log!("collision_vel: {collision_vel:?}");
            log!("pos_correct: {pos_correct:?}");

            log!("cell1: {cell1:?}");
            log!("cell2: {cell2:?}");
        }
    }

    pub fn tick(&mut self) {
        self.render();

        for _ in 0..((1.0 / self.dt) as usize) {
            self.log_cells();

            self.calc_forces();
            self.update_vel();

            self.calc_collisions();
            self.update_pos();
            self.zero_forces();
        }

        //log!("{}", self.render());
    }

    fn add_cell(&mut self, cell: Cell) {
        self.cells.push(Cell {
            index: self.cells.len(),
            ..cell
        });
    }

    fn wall_cell(&self, x: f64, y: f64) -> Cell {
        Cell {
            index: 0,
            color: Color { r: 150, g: 0, b: 0 },
            inertia: Inertia {
                velocity: V2 { x: 0.0, y: 0.0 },
                force: V2 { x: 0.0, y: 0.0 },
                pos: V2 { x, y },
                mass: 0,
            },
        }
    }

    pub fn click(&mut self, x: u32, y: u32) {
        self.add_cell(Cell {
            index: 0,
            color: Color {
                r: ((10 * x) % 255) as u8,
                g: 150,
                b: ((155 * y) % 255) as u8,
            },
            inertia: Inertia {
                velocity: V2 { x: 0.0, y: 0.0 },
                force: V2 { x: 0.0, y: 0.0 },
                pos: V2 {
                    x: x as f64,
                    y: y as f64,
                },
                mass: 1,
            },
        });
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let mut uni = Universe {
            pixels_width: width,
            pixels_height: height,
            cells: Vec::new(),
            grid: Grid::new(width as usize, height as usize),
            pixels: {
                let mut pixels = Vec::with_capacity((width * height) as usize);
                pixels.resize((width * height) as usize, 0xFFFFFF);
                pixels
            },
            gravity: V2 { x: 0.0, y: 0.1 },
            dt: 0.01,
        };

        for x in width / 4..(3 * width / 4) {
            uni.add_cell(uni.wall_cell(x as f64, (height / 2) as f64));
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
            let x = round(cell.inertia.pos.x);
            let y = round(cell.inertia.pos.y);
            // out of the screen bounds
            if x < 0 || x >= (self.pixels_width as i32) || y < 0 || y >= (self.pixels_height as i32)
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
