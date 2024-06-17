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

const MAX_CELLS: usize = 4096;

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

#[derive(Default, Hash, Eq, Clone, Copy, Debug, PartialEq)]
struct CellIndex {
    index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    index: CellIndex,
    color: Color,
    inertia: Inertia,
    collisions: usize,
}

impl Cell {
    fn make_cell_static(&mut self) {
        self.inertia.velocity = V2::zero();
        self.inertia.pos = V2 {
            x: round(self.inertia.pos.x) as f64,
            y: round(self.inertia.pos.y) as f64,
        };
        self.inertia.mass = 0;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats {
    ticks: usize,
    cells_count: usize,
    collisions_count: usize,
    collision_pairs_tested: usize,
}

#[wasm_bindgen]
impl Stats {
    pub fn zero() -> Stats {
        Stats {
            ticks: 0,
            cells_count: 0,
            collisions_count: 0,
            collision_pairs_tested: 0,
        }
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    pub fn cells_count(&self) -> usize {
        self.cells_count
    }

    pub fn collisions_count(&self) -> usize {
        self.collisions_count
    }

    pub fn collision_pairs_tested(&self) -> usize {
        self.collision_pairs_tested
    }
}

#[wasm_bindgen]
pub struct Universe {
    pixels_width: u32,
    pixels_height: u32,
    cells: Vec<Cell>,
    moving_cells: Vec<CellIndex>,
    grid: Grid<CellIndex>,

    // transient data:
    collisions_list: Vec<(CellIndex, CellIndex)>,
    collisions_map: IntPairSet,

    pixels: Vec<u32>,
    gravity: V2,
    dt: f64,
    stats: Stats,
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
            cell.inertia.force = V2::zero();
        }
    }

    fn update_pos(&mut self) {
        self.grid.clear();

        for cell_index in &self.moving_cells {
            let cell = &mut self.cells[cell_index.index];
            cell.inertia.pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(self.dt));
        }
        self.moving_cells.clear();
        for cell in &self.cells {
            if !self.is_in_bounds(cell.inertia.pos) {
                continue;
            }
            //} && (cell.inertia.velocity.len() > self.velocity_threshold())
            if cell.inertia.mass > 0 {
                self.moving_cells.push(cell.index);
            }
            // cell is in bounds
            // store in the grid
            self.grid.put(
                cell.inertia.pos.x as usize,
                cell.inertia.pos.y as usize,
                cell.index,
            );
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
    fn max_velocity(&self) -> V2 {
        V2 {
            x: 0.5 / self.dt,
            y: 0.5 / self.dt,
        }
    }

    fn update_vel(&mut self) {
        let max_vel = self.max_velocity();
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
                    .min(max_vel)
            }
        }
    }

    fn is_in_bounds(&self, pos: V2) -> bool {
        pos.x >= 0.0
            && pos.x < self.pixels_width as f64
            && pos.y >= 0.0
            && pos.y < self.pixels_height as f64
    }

    fn velocity_threshold(&self) -> f64 {
        self.dt / 10.0
    }

    fn collect_collisions(&mut self) {
        self.collisions_map.clear();
        self.collisions_list.clear();

        for cell1_idx in self.moving_cells.iter() {
            let cell1 = &self.cells[cell1_idx.index];

            let (neighbors_count, neighbors) = self
                .grid
                .get(cell1.inertia.pos.x as usize, cell1.inertia.pos.y as usize);
            for cell2_idx in &neighbors[0..neighbors_count] {
                if *cell1_idx == *cell2_idx {
                    continue;
                }

                if self
                    .collisions_map
                    .contains(cell1_idx.index, cell2_idx.index)
                {
                    continue;
                }
                self.collisions_map.put(cell1_idx.index, cell2_idx.index);

                self.stats.collision_pairs_tested += 1;

                let cell2 = &self.cells[cell2_idx.index];
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

                self.collisions_list.push((*cell1_idx, *cell2_idx));

                log!("collision: {key:?} {normal:?} {dot:?}");
                log!("cell1: {cell1:?}");
                log!("cell2: {cell2:?}");

                // println!("collisions: {:?}", collisions);
            }
        }
    }

    fn calc_collisions(&mut self) {
        self.collect_collisions();
        self.stats.collisions_count += self.collisions_list.len();
        for (cell1_idx, cell2_idx) in self.collisions_list.iter() {
            let cell2 = self.cells[cell2_idx.index];
            let cell1 = self.cells[cell1_idx.index];

            // static cell is involved, make them both static
            if ((cell1.inertia.mass == 0) || (cell2.inertia.mass == 0))
                && (cell1.inertia.velocity.magnitude_sqr() < self.velocity_threshold())
                && (cell2.inertia.velocity.magnitude_sqr() < self.velocity_threshold())
            {
                self.cells[cell1_idx.index].make_cell_static();
                self.cells[cell2_idx.index].make_cell_static();

                continue;
            }

            let rel_velocity = cell1.inertia.velocity.minus(cell2.inertia.velocity);
            let normal = cell1.inertia.pos.minus(cell2.inertia.pos);
            // coefficient of restitution
            let e = 0.9;
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
                .cmul(0.4);

            let impulse = collision_vel / (im1 + im2);

            {
                let cell = &mut self.cells[cell1_idx.index];
                cell.inertia.velocity = cell1
                    .inertia
                    .velocity
                    .plus(normal_direction.cmul(impulse * im1));
                cell.inertia.pos = cell1.inertia.pos.plus(pos_correct.cmul(im1));
                cell.collisions += 1;
            }
            {
                let cell = &mut self.cells[cell2_idx.index];
                cell.inertia.velocity = cell2
                    .inertia
                    .velocity
                    .minus(normal_direction.cmul(impulse * im2));
                cell.inertia.pos = cell2.inertia.pos.minus(pos_correct.cmul(im2));
                cell.collisions += 1;
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
        self.stats.ticks += 1;
        self.render();
        self.reset_cells();

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
        if self.cells.len() == MAX_CELLS {
            return;
        }
        // don't allow adding too many cells in the same region
        let (neighbors, _) = self
            .grid
            .get(cell.inertia.pos.x as usize, cell.inertia.pos.y as usize);
        if neighbors > 6 {
            return;
        }

        self.stats.cells_count += 1;
        let index = CellIndex {
            index: self.cells.len(),
        };
        self.cells.push(Cell {
            index,
            collisions: 0,
            ..cell
        });
        self.grid.put(
            cell.inertia.pos.x as usize,
            cell.inertia.pos.y as usize,
            cell.index,
        );
        self.moving_cells.push(index);
    }

    fn wall_cell(&self, x: f64, y: f64) -> Cell {
        Cell {
            index: CellIndex { index: 0 },
            color: Color { r: 150, g: 0, b: 0 },
            inertia: Inertia {
                velocity: V2::zero(),
                force: V2::zero(),
                pos: V2 { x, y },
                mass: 0,
            },
            collisions: 0,
        }
    }

    pub fn click(&mut self, x: u32, y: u32) {
        // unstick some cells
        let (neighbors_count, neighbors) = self.grid.get(x as usize, y as usize);
        let w = self.pixels_width as f64;
        for cell_idx in &neighbors[0..neighbors_count] {
            let cell = &mut self.cells[cell_idx.index];
            if cell.inertia.mass > 0 {
                continue;
            }
            cell.inertia.mass = 1;
            cell.inertia.velocity = V2 {
                x: 2.0 * (x as f64 - w / 2.0) / w,
                y: -1.0, //(cell_idx.index % 10 - 5) as f64 / 10000.0 * self.dt,
            };
        }

        // add a new cell
        self.add_cell(Cell {
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
                    x: x as f64,
                    y: y as f64,
                },
                mass: 1,
            },
            collisions: 0,
        });
    }

    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let mut uni = Universe {
            pixels_width: width,
            pixels_height: height,
            cells: Vec::new(),
            moving_cells: Vec::new(),
            grid: Grid::new(width as usize, height as usize),
            pixels: {
                let mut pixels = Vec::with_capacity((width * height) as usize);
                pixels.resize((width * height) as usize, 0xFFFFFF);
                pixels
            },
            gravity: V2 { x: 0.0, y: 0.1 },
            dt: 0.01,
            stats: Stats::zero(),

            collisions_list: Vec::new(),
            collisions_map: IntPairSet::new(MAX_CELLS),
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
            self.pixels[pixel_idx] = if cell.collisions > 0 {
                0xFF0000
            } else {
                cell.color.to_u32()
            }
        }
    }
    fn reset_cells(&mut self) {
        for cell in &mut self.cells {
            cell.collisions = 0;
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

    pub fn cells_count(&self) -> usize {
        self.cells.len()
    }

    pub fn stats(&mut self) -> Stats {
        let res = self.stats;
        self.stats = Stats::zero();
        res
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
