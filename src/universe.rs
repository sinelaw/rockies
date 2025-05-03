use std::cell::RefCell;
use std::rc::Rc;

use crate::assets;
use crate::color::Color;
use crate::inertia::Inertia;
use crate::multigrid::{CellIndex, GridIndex, MultiGrid, UniverseGrid};
use crate::v2::{V2i, V2};

use noise::Vector2;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};

use fnv::{FnvHashMap, FnvHashSet};
use wasm_bindgen::prelude::*;

extern crate web_sys;

const MAX_CELLS: usize = 4096 * 16;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        // web_sys::console::log_1(&format!( $( $t )* ).into())
        // println!( $( $t )* );
    };
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

    pub fn render(
        &self,
        pixels: &mut Vec<u32>,
        offset: V2i,
        buf_width: usize,
        buf_height: usize,
    ) -> () {
        let hammy_0: (usize, usize, &[Color]) = assets::HAMMY_0;
        let hammy_1: (usize, usize, &[Color]) = assets::HAMMY_1;
        let hammy_2: (usize, usize, &[Color]) = assets::HAMMY_2;
        let hammies = [hammy_0, hammy_1, hammy_2];
        let (w, h, colors) = hammies[self.frame % 3];

        for x in 0..(w as i32) {
            for y in 0..(h as i32) {
                let py = (offset.y + y) as f64;
                let px = if self.direction >= 0 {
                    (offset.x + x) as f64
                } else {
                    (offset.x + (w as i32 - x - 1)) as f64
                };
                if Self::in_bounds(px, py, buf_width, buf_height) {
                    let c = colors[(x + y * w as i32) as usize];
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
                let posi = pos.round();
                let grid = cells.grids.get(cells.grids.pos_to_index(posi)).unwrap();
                if !grid.is_in_bounds(posi) {
                    continue;
                }
                let player_part = Inertia {
                    pos: pos,
                    ..self.inertia
                };
                let get_res = grid.get(posi);
                for cell_idx in get_res.neighbors {
                    let cell = cell_idx.borrow();
                    let cell_inertia = &cell.inertia;

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

fn clamp_velocity(v: V2) -> V2 {
    let max = V2 { x: 1.0, y: 1.0 };
    let min = V2 { x: -1.0, y: -1.0 };
    return v.min(max).max(min);
}

fn velocity_threshold(dt: f64) -> f64 {
    dt / 5.0
}

pub struct UniverseCells {
    moving_cells: FnvHashSet<Rc<RefCell<Cell>>>,

    grids: MultiGrid<Cell>,
    next_cell_index: usize,

    stats: Stats,
    // transient data:
    collisions_list: Vec<(CellIndex, CellIndex)>,
    collisions_map: FnvHashSet<(CellIndex, CellIndex)>,

    seed: i32,
}

impl UniverseCells {
    fn new(width: usize, height: usize) -> UniverseCells {
        UniverseCells {
            moving_cells: FnvHashSet::default(),

            grids: MultiGrid::new(width, height),
            next_cell_index: 0,
            stats: Stats::zero(),

            collisions_list: Vec::new(),
            collisions_map: FnvHashSet::default(),

            seed: 0,
        }
    }

    fn wall_cell(pos: V2i, color: Color) -> Cell {
        Cell {
            index: CellIndex::default(),
            color: color,
            inertia: Inertia {
                velocity: V2::zero(),
                force: V2::zero(),
                pos: pos.to_v2(),
                mass: 0,
                elasticity: 1.0, // allow other mass to determine
                collision_stats: 0,
            },
        }
    }

    fn ensure_grid(&mut self, grid_index: GridIndex) {
        let width = self.grids.grid_width;
        let height = self.grids.grid_height;
        let is_new = self
            .grids
            .or_insert_with(grid_index, || UniverseGrid::new(grid_index, width, height));
        if is_new {
            self.create_wall_cells(grid_index);
        }
    }

    fn generated_point(&self, pos: V2i) -> f64 {
        let hasher = PermutationTable::new(self.seed as u32);
        // Check for caverns
        let posv = pos.to_v2().cmul(0.01);

        // perlin_2d returns a value in (-1..1)
        let local_seed = perlin_2d(Vector2::new(posv.x, posv.y), &hasher).abs()
            * perlin_2d(Vector2::new(posv.y * 0.3, posv.x * 0.4), &hasher).abs();
        local_seed
    }

    fn create_wall_cells(&mut self, grid_index: GridIndex) {
        let width = self.grids.grid_width;
        let height = self.grids.grid_height;
        let base_pos = grid_index.to_pos(width, height);

        for x in 0..width {
            for y in 0..height {
                let pos = V2i::new(x as i32, y as i32).plus(base_pos);
                let altitude = height as i32 - pos.y;
                let above_ground = altitude > 0;
                if above_ground {
                    // generate "mountains"
                    let val = self.generated_point(V2i::new(pos.x, 0));

                    if val * 100.0 > altitude as f64 {
                        self.add_cell(UniverseCells::wall_cell(
                            pos,
                            Color::hsv(30.0, 1.0, 0.5), // brown
                        ));
                    }
                } else {
                    // below ground
                    let val = self.generated_point(pos);
                    let depth = -altitude as f64;
                    if val < 0.02 + 0.5 / (depth * 0.1) {
                        self.add_cell(UniverseCells::wall_cell(
                            pos,
                            Color::hsv(30.0, 1.0, (1.0 - val) * 0.5), // brown
                        ));
                    }
                }
            }
        }
    }

    pub fn get_range(
        &mut self,
        start_pos: V2i,
        end_pos: V2i,
    ) -> Vec<(V2i, Option<Rc<RefCell<Cell>>>)> {
        self.ensure_grids(start_pos, end_pos);

        let count = (end_pos.x - start_pos.x) * (end_pos.y - start_pos.y);
        let mut result = Vec::with_capacity(count as usize);

        let mut cur_grid: Option<(GridIndex, &UniverseGrid<Cell>)> = Option::None;
        for x in start_pos.x..end_pos.x {
            for y in start_pos.y..end_pos.y {
                let pos = V2i::new(x, y);
                let grid_index = self.grids.pos_to_index(pos);

                // Only lookup grid if grid_index changed
                if cur_grid.map_or(true, |x| x.0 != grid_index) {
                    cur_grid = Some((grid_index, self.grids.get(grid_index).unwrap()));
                }
                let grid = cur_grid.unwrap().1;

                let get_res = grid.get(pos);
                result.push((pos, self.get_cell_at(get_res)));
            }
        }

        result
    }

    fn get_cell_at(&self, get_res: crate::grid::GetResult<'_, Cell>) -> Option<Rc<RefCell<Cell>>> {
        for cell_idx in get_res.value {
            return Some(cell_idx.clone());
        }
        None
    }

    fn ensure_grids(&mut self, start_pos: V2i, end_pos: V2i) {
        // Pre-ensure all grids we'll need
        let width = self.grids.grid_width;
        let height = self.grids.grid_height;
        for x in (start_pos.x..(end_pos.x + width as i32)).step_by(width) {
            for y in (start_pos.y..(end_pos.y + height as i32)).step_by(height) {
                let pos = V2i::new(x, y);
                let grid_index = self.grids.pos_to_index(pos);
                self.ensure_grid(grid_index);
            }
        }
    }

    fn calc_forces(&mut self, gravity: V2) {
        for cell_idx in self.moving_cells.iter() {
            let mut cell = cell_idx.borrow_mut();
            if cell.inertia.mass > 0 {
                cell.inertia.force = gravity.cmul(cell.inertia.mass as f64);
            }
        }
    }

    fn zero_forces(&mut self) {
        for cell_idx in self.moving_cells.iter() {
            let mut cell = cell_idx.borrow_mut();
            cell.inertia.force = V2::zero();
        }
    }

    fn update_velocity(&mut self, dt: f64) {
        for cell_idx in self.moving_cells.iter() {
            let mut cell = cell_idx.borrow_mut();
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
            let mut cell1 = cell1_idx.borrow();
            let grid_index = self.grids.pos_to_index(cell1.inertia.pos.round());

            if self.grids.get(grid_index).is_none() {
                continue;
            }
            let get_res = self
                .grids
                .get(grid_index)
                .unwrap()
                .get(cell1.inertia.pos.round());
            for cell2_idx in get_res.neighbors {
                if Rc::ptr_eq(cell1_idx, cell2_idx) {
                    continue;
                }

                if !self.collisions_map.insert((cell1_idx, cell2_idx)) {
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
                && (low_velocity_collision(inertia1, inertia2, dt))
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

            self.grids
                .update_cell_pos(*cell1_idx, inertia1.pos.round(), new_inertia1.pos.round());
            self.grids
                .update_cell_pos(*cell2_idx, inertia2.pos.round(), new_inertia2.pos.round());

            Self::update_cell_collision(&mut self.cells, cell1_idx, new_inertia1);
            Self::update_cell_collision(&mut self.cells, cell2_idx, new_inertia2);
        }
    }

    fn update_cell_collision(
        cells: &mut FnvHashMap<CellIndex, Cell>,
        cell_index: &CellIndex,
        new_inertia: Inertia,
    ) {
        let cell = cells.get_mut(cell_index).unwrap();
        cell.inertia = new_inertia;
        cell.inertia.collision_stats += 1;
        log!("index: {cell_index:?}, inertia: {new_inertia:?}");
    }

    fn update_pos(&mut self, dt: f64) {
        // update grid and positions
        for cell_index in &self.moving_cells {
            let cell = self.cells.get_mut(cell_index).unwrap();
            let old_pos = cell.inertia.pos;
            let new_pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(dt));

            // update grid:
            self.grids
                .update_cell_pos(*cell_index, old_pos.round(), new_pos.round());
            // update position:
            cell.inertia.pos = new_pos;
        }

        // Filter out moving cells that have been made static
        // (TODO: some previously static cells may now need to be in moving_cells?)
        self.moving_cells = self
            .moving_cells
            .iter()
            .filter(|cell_idx| {
                self.cells
                    .get(cell_idx)
                    .map(|x| x.inertia.mass > 0)
                    .unwrap_or(false)
            })
            .map(|x| *x)
            .collect();
    }

    pub fn add_cell(&mut self, cell: Cell) {
        if self.cells.len() == MAX_CELLS {
            return;
        }
        let pos = cell.inertia.pos.round();
        let grid_index = self.grids.pos_to_index(pos);
        // don't allow adding too many cells in the same region
        let grid = self.grids.get_mut(grid_index).unwrap();
        let get_res = grid.get(pos);
        if get_res.neighbors.len() > 6 {
            return;
        }

        self.next_cell_index += 1;
        self.stats.cells_count += 1;
        let index = CellIndex {
            index: self.next_cell_index,
        };
        self.cells.insert(index, Cell { index, ..cell });
        self.moving_cells.insert(index);
        grid.put(pos, index);
    }

    fn get_cells(&mut self, center: V2i, radius: usize) -> Vec<CellIndex> {
        let mut res = Vec::new();
        let r = radius as i32;
        for i in -r..r {
            for j in -r..r {
                let ppos = center.plus(V2i::new(i, j));
                let grid_index = self.grids.pos_to_index(ppos);
                self.ensure_grid(grid_index);
                let get_res = self.grids.get(grid_index).unwrap().get(ppos);
                res.extend_from_slice(get_res.neighbors);
            }
        }
        res
    }

    pub fn unstick_cells(&mut self, center: V2i, radius: usize) {
        for cell_idx in self.get_cells(center, radius) {
            let cell = self.cells.get_mut(&cell_idx).unwrap();
            if cell.inertia.mass > 0 {
                continue;
            }
            cell.unset_static();
            self.moving_cells.insert(cell_idx);
            cell.inertia.velocity = V2 {
                x: 2.0 * (cell_idx.index % 10 - 5) as f64 / 10.0,
                y: -1.0, //(cell_idx.index % 10 - 5) as f64 / 10000.0 * self.dt,
            };
        }
    }

    pub fn remove_cell(&mut self, ppos: V2i) {
        let grid_index = self.grids.pos_to_index(ppos);
        self.ensure_grid(grid_index);

        let values: Vec<CellIndex> = self
            .grids
            .get(grid_index)
            .unwrap()
            .get(ppos)
            .value
            .iter()
            .map(|x| *x)
            .collect();

        for cell_idx in values {
            self.cells.remove(&cell_idx);
            self.moving_cells.remove(&cell_idx);
            self.grids
                .get_mut(grid_index)
                .unwrap()
                .remove(ppos, cell_idx);
        }
    }

    fn reset_cell_stats(&mut self) {
        for (_, cell) in &mut self.cells {
            cell.inertia.collision_stats = 0;
        }
    }

    fn drop_far_cells(&mut self, center: V2) {
        let drop_radius = 2;
        let far_grids = self.grids.get_far_grids(center.round(), drop_radius);

        if far_grids.len() == 0 {
            return;
        }

        for grid_index in far_grids.iter() {
            let grid = self.grids.get_mut(*grid_index).unwrap();
            let grid_origin = grid_index.to_pos(grid.width, grid.height);
            for x in 0..grid.width {
                for y in 0..grid.height {
                    let values = grid
                        .get(V2i::new(x as i32, y as i32).plus(grid_origin))
                        .value;
                    for cell_index in values {
                        self.cells.remove(&cell_index);
                        self.moving_cells.remove(&cell_index);
                    }
                }
            }
        }
        //println!("removed {} cells", removed_cells);
        for grid_index in far_grids {
            self.grids.drop_grid(grid_index);
        }
    }
}

fn low_velocity_collision(inertia1: &Inertia, inertia2: &Inertia, dt: f64) -> bool {
    (inertia1.velocity.magnitude_sqr() < velocity_threshold(dt))
        && (inertia2.velocity.magnitude_sqr() < velocity_threshold(dt))
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

    fn drop_far_cells(&mut self) {
        self.cells.drop_far_cells(self.player.inertia.pos);
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
            self.drop_far_cells();
            self.cells.update_pos(self.dt);
            self.zero_forces();
        }

        //log!("{}", self.render());
    }

    pub fn stats(&mut self) -> Stats {
        self.cells.stats.get_and_reset()
    }

    pub fn new(width: usize, height: usize) -> Universe {
        Universe {
            cells: UniverseCells::new(width, height),
            gravity: V2 { x: 0.0, y: 0.1 },
            dt: 0.01,

            player: Player::new(1, 1),
        }
    }
}
