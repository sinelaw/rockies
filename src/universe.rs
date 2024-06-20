use std::iter::FromIterator;

use crate::assets;
use crate::color::Color;
use crate::grid::Grid;
use crate::inertia::Inertia;
use crate::v2::{V2i, V2};
use fnv::{FnvHashMap, FnvHashSet};
use wasm_bindgen::prelude::*;

extern crate web_sys;

const MAX_CELLS: usize = 4096;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        // web_sys::console::log_1(&format!( $( $t )* ).into())
        // println!( $( $t )* );
    };
}

#[derive(Default, Hash, Eq, Clone, Copy, Debug, PartialEq)]
pub struct CellIndex {
    pub index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub index: CellIndex,
    pub color: Color,
    pub inertia: Inertia,
}

impl Cell {
    fn set_static(&mut self) {
        self.inertia.velocity = V2::zero();
        self.inertia.pos = self.inertia.pos.round().to_v2();
        self.inertia.mass = 0;
    }
    fn unset_static(&mut self) {
        self.inertia.mass = 1;
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
    pub fn get_and_reset(&mut self) -> Stats {
        let res = self.clone();
        *self = Stats::zero();
        res
    }

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

pub struct Player {
    pub w: usize,
    pub h: usize,
    pub inertia: Inertia,

    pub frame: usize,
    pub direction: i32,
}

impl Player {
    fn new(x: usize, y: usize) -> Self {
        let (w, h, _): (usize, usize, &[Color]) = assets::HAMMY_0;

        Player {
            w,
            h,
            inertia: Inertia {
                velocity: V2::zero(),
                force: V2::zero(),
                pos: V2::new(x as f64, y as f64),
                mass: 100,
                elasticity: 0.0,
                collision_stats: 0,
            },

            direction: 1,
            frame: 0,
        }
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
    }

    pub fn move_left(&mut self) {
        self.inertia.velocity.x = -0.5;
        self.direction = -1;
        self.next_frame();
    }

    pub fn move_right(&mut self) {
        self.inertia.velocity.x = 0.5;
        self.direction = 1;
        self.next_frame();
    }

    pub fn move_up(&mut self) {
        self.inertia.velocity.y = -0.5;
        self.next_frame();
    }

    pub fn move_down(&mut self) {
        self.inertia.velocity.y = 0.5;
        self.next_frame();
    }

    pub fn mouth_pos(&self) -> V2 {
        V2::new(
            self.inertia.pos.x
                + (if self.direction >= 0 {
                    self.w as f64
                } else {
                    -1.0
                }),
            self.inertia.pos.y + (self.h / 2 - 1) as f64,
        )
    }

    pub fn render(&self, pixels: &mut Vec<u32>, buf_width: usize, buf_height: usize) -> () {
        let hammy_0: (usize, usize, &[Color]) = assets::HAMMY_0;
        let hammy_1: (usize, usize, &[Color]) = assets::HAMMY_1;
        let hammy_2: (usize, usize, &[Color]) = assets::HAMMY_2;
        let hammies = [hammy_0, hammy_1, hammy_2];
        let (w, h, colors) = hammies[self.frame % 3];

        for x in 0..w {
            for y in 0..h {
                let py = self.inertia.pos.y + y as f64;
                let px = if self.direction >= 0 {
                    self.inertia.pos.x + x as f64
                } else {
                    self.inertia.pos.x + (w - x - 1) as f64
                };
                if Self::in_bounds(px, py, buf_width, buf_height) {
                    let c = colors[x + y * w];
                    if c.r == 0 && c.g == 0 && c.b == 0 {
                        continue;
                    }
                    pixels[(py as usize) * buf_width + (px as usize)] = c.to_u32();
                }
            }
        }
    }

    fn in_bounds(x: f64, y: f64, buf_width: usize, buf_height: usize) -> bool {
        x >= 0.0 && y >= 0.0 && x < buf_width as f64 && y < buf_height as f64
    }

    pub fn update_pos(&mut self, cells: &UniverseCells, dt: f64) {
        self.inertia = self.get_next_player_inertia(cells, dt);
    }

    fn get_next_player_inertia(&self, cells: &UniverseCells, dt: f64) -> Inertia {
        //log!("player pos: {:?}", self.inertia.pos);
        let new_player_pos = self.inertia.pos.plus(self.inertia.velocity.cmul(dt));
        //if new_player_pos.round() != self.inertia.pos.round() {
        // position changed, check if colliding
        for x in 0..self.w {
            for y in 0..self.h {
                let pos = V2 {
                    x: new_player_pos.x + x as f64,
                    y: new_player_pos.y + y as f64,
                };
                if !cells.grid.is_in_bounds(pos.round()) {
                    continue;
                }
                let player_part = Inertia {
                    pos: pos,
                    ..self.inertia
                };
                let get_res = cells.grid.get(pos.round());
                for cell_idx in get_res.neighbors {
                    let cell_inertia = &cells.cells[cell_idx].inertia;

                    if Inertia::is_collision(&player_part, cell_inertia) {
                        return Inertia {
                            velocity: V2::zero(),
                            pos: self.inertia.pos.round().to_v2(),
                            ..self.inertia
                        };
                    }
                }
            }
        }
        //}
        return Inertia {
            pos: new_player_pos,
            ..self.inertia
        };
    }

    pub fn calc_forces(&mut self, gravity: V2) {
        self.inertia.force = self
            .inertia
            .force
            .plus(gravity.cmul(self.inertia.mass as f64));
    }

    pub fn update_velocity(&mut self, dt: f64) {
        self.inertia.velocity = self
            .inertia
            .velocity
            .plus(self.inertia.force.cdiv(self.inertia.mass as f64).cmul(dt));
    }
}

// Keeps track of the visible part of the world
pub struct UniverseGrid {
    pub width: usize,
    pub height: usize,

    offset: V2i,
    grid: Grid<CellIndex>,
}

impl UniverseGrid {
    fn is_in_bounds(&self, pos: V2i) -> bool {
        let relative_pos = pos.minus(self.offset);
        relative_pos.x >= 0
            && relative_pos.y >= 0
            && relative_pos.x < self.width as i32
            && relative_pos.y < self.height as i32
    }

    fn set_offset(&mut self, offset: V2i) {
        self.offset = offset;
    }

    fn update_cell_pos(&mut self, cell_idx: CellIndex, old_pos: V2i, new_pos: V2i) {
        // update grid:
        if self.is_in_bounds(old_pos) {
            self.remove(old_pos, cell_idx);
        }
        if self.is_in_bounds(new_pos) {
            self.put(new_pos, cell_idx);
        }
    }

    pub fn remove(&mut self, pos: V2i, cell_idx: CellIndex) {
        assert!(self.is_in_bounds(pos));
        self.grid
            .remove((pos.x) as usize, (pos.y) as usize, cell_idx)
    }

    pub fn put(&mut self, pos: V2i, cell_idx: CellIndex) {
        assert!(self.is_in_bounds(pos));
        self.grid.put((pos.x) as usize, (pos.y) as usize, cell_idx)
    }

    pub fn get(&self, pos: V2i) -> crate::grid::GetResult<CellIndex> {
        assert!(self.is_in_bounds(pos));
        self.grid.get(pos.x as usize, pos.y as usize)
    }
}

fn clamp_velocity(v: V2) -> V2 {
    let max = V2 { x: 1.0, y: 1.0 };
    let min = V2 { x: -1.0, y: -1.0 };
    return v.min(max).max(min);
}

fn velocity_threshold(dt: f64) -> f64 {
    dt / 10.0
}

pub struct UniverseCells {
    pub cells: FnvHashMap<CellIndex, Cell>,
    moving_cells: Vec<CellIndex>,
    grid: UniverseGrid,

    next_cell_index: usize,

    stats: Stats,
    // transient data:
    collisions_list: Vec<(CellIndex, CellIndex)>,
    collisions_map: FnvHashSet<(CellIndex, CellIndex)>,
}

impl UniverseCells {
    fn new(width: usize, height: usize) -> UniverseCells {
        UniverseCells {
            cells: FnvHashMap::default(),
            moving_cells: Vec::default(),
            grid: UniverseGrid {
                grid: Grid::new(width as usize, height as usize),
                width: width,
                height: height,
                offset: V2i::zero(),
            },

            next_cell_index: 0,
            stats: Stats::zero(),

            collisions_list: Vec::new(),
            collisions_map: FnvHashSet::default(),
        }
    }

    pub fn get(&self, pos: V2i) -> crate::grid::GetResult<CellIndex> {
        self.grid.get(pos)
    }

    fn calc_forces(&mut self, gravity: V2) {
        for cell_idx in self.moving_cells.iter() {
            let cell = &mut self.cells.get_mut(&cell_idx).unwrap();
            if cell.inertia.mass > 0 {
                cell.inertia.force = gravity.cmul(cell.inertia.mass as f64);
            }
        }
    }

    fn zero_forces(&mut self) {
        for cell_idx in self.moving_cells.iter() {
            let cell = &mut self.cells.get_mut(&cell_idx).unwrap();
            cell.inertia.force = V2::zero();
        }
    }

    fn update_velocity(&mut self, dt: f64) {
        for cell_idx in self.moving_cells.iter() {
            let cell = &mut self.cells.get_mut(&cell_idx).unwrap();
            if cell.inertia.mass > 0 {
                cell.inertia.velocity = clamp_velocity(
                    cell.inertia
                        .velocity
                        .plus(cell.inertia.force.cdiv(cell.inertia.mass as f64).cmul(dt)),
                );
            }
        }
    }

    fn collect_collisions(&mut self) {
        self.collisions_map.clear();
        self.collisions_list.clear();

        for cell1_idx in self.moving_cells.iter() {
            let cell1 = &self.cells[cell1_idx];

            if !self.grid.is_in_bounds(cell1.inertia.pos.round()) {
                continue;
            }
            let get_res = self
                .grid
                .grid
                .get(cell1.inertia.pos.x as usize, cell1.inertia.pos.y as usize);
            for cell2_idx in get_res.neighbors {
                if *cell1_idx == *cell2_idx {
                    continue;
                }

                if !self.collisions_map.insert((*cell1_idx, *cell2_idx)) {
                    continue;
                }

                self.stats.collision_pairs_tested += 1;

                let cell2 = &self.cells[cell2_idx];
                let inertia1 = &cell1.inertia;
                let inertia2 = &cell2.inertia;

                if Inertia::is_collision(inertia1, inertia2) {
                    self.collisions_list.push((*cell1_idx, *cell2_idx));
                }

                // log!("cell1: {cell1:?}");
                // log!("cell2: {cell2:?}");

                // println!("collisions: {:?}", collisions);
            }
        }
    }

    fn calc_collisions(&mut self, dt: f64) {
        self.collect_collisions();
        self.stats.collisions_count += self.collisions_list.len();
        for (cell1_idx, cell2_idx) in self.collisions_list.iter() {
            let inertia2 = &self.cells[cell2_idx].inertia;
            let inertia1 = &self.cells[cell1_idx].inertia;

            let mass1 = inertia1.mass;
            let mass2 = inertia2.mass;
            // static cell is involved, make them both static
            if ((inertia1.mass == 0) || (inertia2.mass == 0))
                && (inertia1.velocity.magnitude_sqr() < velocity_threshold(dt))
                && (inertia2.velocity.magnitude_sqr() < velocity_threshold(dt))
            {
                if mass1 > 0 {
                    self.cells.get_mut(cell1_idx).unwrap().set_static();
                }
                if mass2 > 0 {
                    self.cells.get_mut(cell2_idx).unwrap().set_static();
                }

                continue;
            }

            let (new_inertia1, new_inertia2) = Inertia::collide(inertia1, inertia2);

            self.grid
                .update_cell_pos(*cell1_idx, inertia1.pos.round(), new_inertia1.pos.round());
            self.grid
                .update_cell_pos(*cell2_idx, inertia2.pos.round(), new_inertia2.pos.round());

            self.cells.get_mut(cell1_idx).unwrap().inertia = new_inertia1;
            self.cells.get_mut(cell2_idx).unwrap().inertia = new_inertia2;
        }
    }

    fn update_pos(&mut self, dt: f64) {
        // some previously static cells may now need to be in moving_cells
        self.moving_cells.clear();
        for (_, cell) in &self.cells {
            if !self.grid.is_in_bounds(cell.inertia.pos.round()) {
                continue;
            }
            //} && (cell.inertia.velocity.len() > self.velocity_threshold())
            if cell.inertia.mass > 0 {
                self.moving_cells.push(cell.index);
            }
        }

        // update grid and positions
        for cell_index in &self.moving_cells {
            let cell = self.cells.get(cell_index).unwrap();
            let old_pos = cell.inertia.pos;
            let new_pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(dt));

            // update grid:
            self.grid
                .update_cell_pos(*cell_index, old_pos.round(), new_pos.round());
            // update position:
            self.cells.get_mut(cell_index).unwrap().inertia.pos = new_pos;
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        if self.cells.len() == MAX_CELLS {
            return;
        }
        // don't allow adding too many cells in the same region
        let get_res = self
            .grid
            .grid
            .get(cell.inertia.pos.x as usize, cell.inertia.pos.y as usize);
        if get_res.neighbors.len() > 6 {
            return;
        }

        self.next_cell_index += 1;
        self.stats.cells_count += 1;
        let index = CellIndex {
            index: self.next_cell_index,
        };
        self.cells.insert(index, Cell { index, ..cell });
        self.grid.put(cell.inertia.pos.round(), index);
        self.moving_cells.push(index);
    }

    fn get_cells(&self, x: usize, y: usize, radius: usize) -> Vec<CellIndex> {
        let mut res = Vec::new();
        let r = radius as i32;
        for i in -r..r {
            for j in -r..r {
                let ppos = V2i::new(x as i32 + i, y as i32 + j);
                if !self.grid.is_in_bounds(ppos) {
                    continue;
                }
                let get_res = self.grid.get(ppos);
                res.extend_from_slice(get_res.neighbors);
            }
        }
        res
    }

    pub fn unstick_cells(&mut self, x: usize, y: usize, radius: usize) {
        let w = self.grid.width as f64;
        for cell_idx in self.get_cells(x, y, radius) {
            let cell = self.cells.get_mut(&cell_idx).unwrap();
            if cell.inertia.mass > 0 {
                continue;
            }
            cell.unset_static();
            self.moving_cells.push(cell_idx);
            cell.inertia.velocity = V2 {
                x: 2.0 * (x as f64 - w / 2.0) / w,
                y: -1.0, //(cell_idx.index % 10 - 5) as f64 / 10000.0 * self.dt,
            };
        }
    }

    pub fn remove_cells(&mut self, x: usize, y: usize, radius: usize) {
        let cells_to_remove: FnvHashSet<CellIndex> = FnvHashSet::from_iter(
            self.get_cells(x, y, radius)
                .iter()
                // don't remove cells that may be interacting (in moving_cells)
                .filter(|cell_idx| self.cells.get_mut(&cell_idx).unwrap().inertia.mass == 0)
                .map(|cell_idx| *cell_idx),
        );
        for cell_idx in cells_to_remove {
            let cell = self.cells.get_mut(&cell_idx).unwrap();
            self.grid.remove(cell.inertia.pos.round(), cell_idx);
            self.cells.remove(&cell_idx);
            log!("removed: {cell_idx:?}");
        }
    }

    fn reset_cell_stats(&mut self) {
        for (_, cell) in &mut self.cells {
            cell.inertia.collision_stats = 0;
        }
    }
}

pub struct Universe {
    gravity: V2,
    dt: f64,
    pub cells: UniverseCells,

    pub player: Player,
}

impl Universe {
    fn calc_forces(&mut self) {
        self.cells.calc_forces(self.gravity);
        self.player.calc_forces(self.gravity);
    }

    fn zero_forces(&mut self) {
        self.player.inertia.force = V2::zero();
        self.cells.zero_forces();
    }

    fn update_velocity(&mut self) {
        self.cells.update_velocity(self.dt);
        self.player.update_velocity(self.dt);
    }

    pub fn tick(&mut self) {
        self.cells.stats.ticks += 1;
        self.cells.reset_cell_stats();

        for _ in 0..((1.0 / self.dt) as usize) {
            //self.log_cells();

            self.calc_forces();
            self.update_velocity();

            self.cells.calc_collisions(self.dt);

            self.player.update_pos(&self.cells, self.dt);
            self.cells.update_pos(self.dt);
            self.zero_forces();
        }

        //log!("{}", self.render());
    }

    pub fn stats(&mut self) -> Stats {
        self.cells.stats.get_and_reset()
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
                elasticity: 1.0, // allow other mass to determine
                collision_stats: 0,
            },
        }
    }
    pub fn new(width: usize, height: usize) -> Universe {
        let mut uni = Universe {
            cells: UniverseCells::new(width, height),
            gravity: V2 { x: 0.0, y: 0.1 },
            dt: 0.01,

            player: Player::new(10, 10),
        };

        for x in width / 4..(3 * width / 4) {
            uni.cells
                .add_cell(uni.wall_cell(x as f64, (height / 2) as f64));
        }
        for x in 0..width {
            uni.cells.add_cell(uni.wall_cell(x as f64, 0.0));
            uni.cells
                .add_cell(uni.wall_cell(x as f64, (height - 1) as f64));
        }
        for y in 0..height {
            uni.cells.add_cell(uni.wall_cell(0.0, y as f64));
            uni.cells
                .add_cell(uni.wall_cell((width - 1) as f64, y as f64));
        }

        uni
    }
}
