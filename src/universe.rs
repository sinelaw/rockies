use std::cell::RefCell;
use std::rc::Rc;

use crate::assets;
use crate::color::Color;
use crate::generator::Generator;
use crate::grid::GridCellRef;
use crate::inertia::Inertia;
use crate::multigrid::{CellIndex, GridIndex, MultiGrid, UniverseGrid};
use crate::v2::{V2i, V2};

use crate::log::log;

use fnv::{FnvHashMap, FnvHashSet};
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cell {
    pub index: CellIndex,
    pub color: Color,
    pub inertia: Inertia,
}

pub const ELASTICITY: f64 = 0.2;

impl Cell {
    fn set_static(&mut self) {
        self.inertia.velocity = V2::zero();
        self.inertia.pos = self.inertia.pos.round().to_v2();
        self.inertia.mass = 0;
        self.inertia.collision_stats = 0;
    }
    fn unset_static(&mut self) {
        self.inertia.mass = 1;
        self.inertia.collision_stats = 0;
        self.inertia.elasticity = ELASTICITY;
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
    pub life: u32,
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
            life: u32::MAX,
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
        is_dig_mode: bool,
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
                    let final_color = if is_dig_mode {
                        // make the color a bit darker
                        c.mix(0.9, 0.7, 0.7)
                    } else {
                        let sickness = self.life as f64 / u32::MAX as f64;
                        c.mix(sickness, 1.0, sickness)
                    };
                    pixels[(py as usize) * buf_width + (px as usize)] = final_color.to_u32();
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
    dt / 2.0
}

pub struct UniverseCells {
    moving_cells: FnvHashMap<CellIndex, GridCellRef<Cell>>,

    grids: MultiGrid<Cell>,
    generator: Generator,
    next_cell_index: usize,

    stats: Stats,
    // transient data:
    collisions_list: Vec<(GridCellRef<Cell>, GridCellRef<Cell>)>,
    collisions_map: FnvHashSet<(CellIndex, CellIndex)>,
}

impl UniverseCells {
    fn new(width: usize, height: usize) -> UniverseCells {
        UniverseCells {
            moving_cells: FnvHashMap::default(),
            generator: Generator::new(0 as u32),

            grids: MultiGrid::new(width, height),
            next_cell_index: 0,
            stats: Stats::zero(),

            collisions_list: Vec::new(),
            collisions_map: FnvHashSet::default(),
        }
    }

    pub fn ensure_grid(&mut self, grid_index: GridIndex) {
        let width = self.grids.grid_width;
        let height = self.grids.grid_height;
        let generator = &mut self.generator;
        let (is_new, grid) = self
            .grids
            .or_insert_with(grid_index, || UniverseGrid::new(grid_index, 64, 64));
        if is_new {
            generator.generate_pristine_grid(grid, grid_index, width, height)
        }
    }

    fn load_from_storage(&mut self, grid_index: GridIndex, grid: UniverseGrid<Cell>) {
        // Load a grid from storage, if it exists
        self.grids.insert(grid_index, grid);
    }

    pub fn get_range(
        &mut self,
        start_pos: V2i,
        end_pos: V2i,
    ) -> Vec<(V2i, Vec<GridCellRef<Cell>>)> {
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

                let mut cur_res = Vec::new();
                for cell_ref in grid.get(pos).value.iter() {
                    cur_res.push(cell_ref.clone());
                }
                result.push((pos, cur_res));
            }
        }

        result
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
        for (_cell_idx, cell_ref) in self.moving_cells.iter() {
            let mut cell = cell_ref.borrow_mut();
            if cell.inertia.mass > 0 {
                cell.inertia.force = gravity.cmul(cell.inertia.mass as f64);
            }
        }
    }

    fn zero_forces(&mut self) {
        for (_cell_idx, cell_ref) in self.moving_cells.iter() {
            let mut cell = cell_ref.borrow_mut();
            cell.inertia.force = V2::zero();
        }
    }

    fn update_velocity(&mut self, dt: f64) {
        for (_cell_idx, cell_ref) in self.moving_cells.iter() {
            let mut cell = cell_ref.borrow_mut();
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

        for (_cell1_idx, cell1_ref) in self.moving_cells.iter() {
            let cell1 = cell1_ref.borrow();
            let grid_index = self.grids.pos_to_index(cell1.inertia.pos.round());

            if self.grids.get(grid_index).is_none() {
                continue;
            }
            let get_res = self
                .grids
                .get(grid_index)
                .unwrap()
                .get(cell1.inertia.pos.round());
            for cell2_ref in get_res.neighbors {
                if Rc::ptr_eq(cell1_ref, cell2_ref) {
                    continue;
                }

                let cell2 = cell2_ref.borrow();
                if !self.collisions_map.insert((cell1.index, cell2.index)) {
                    continue;
                }

                self.stats.collision_pairs_tested += 1;

                let inertia1 = &cell1.inertia;
                let inertia2 = &cell2.inertia;

                if Inertia::is_collision(inertia1, inertia2) {
                    self.collisions_list
                        .push((cell1_ref.clone(), cell2_ref.clone()));
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
            let mut cell1 = cell1_idx.borrow_mut();
            let mut cell2 = cell2_idx.borrow_mut();

            let inertia2 = &cell2.inertia;
            let inertia1 = &cell1.inertia;

            let mass1 = inertia1.mass;
            let mass2 = inertia2.mass;
            // static cell is involved, make them both static
            if ((inertia1.mass == 0) || (inertia2.mass == 0))
                && (low_velocity_collision(inertia1, inertia2, dt))
            {
                if mass1 > 0 {
                    cell1.set_static();
                }
                if mass2 > 0 {
                    cell2.set_static();
                }

                continue;
            }

            let (new_inertia1, new_inertia2) = Inertia::collide(inertia1, inertia2);

            self.grids
                .update_cell_pos(cell1_idx, inertia1.pos.round(), new_inertia1.pos.round());
            self.grids
                .update_cell_pos(cell2_idx, inertia2.pos.round(), new_inertia2.pos.round());

            Self::update_cell_collision(&mut cell1, new_inertia1);
            Self::update_cell_collision(&mut cell2, new_inertia2);
        }
    }

    fn update_cell_collision(cell: &mut Cell, new_inertia: Inertia) {
        cell.inertia = new_inertia;
        cell.inertia.collision_stats += 1;
        if cell.inertia.collision_stats > 1000 {
            // dumpen highly colliding cells
            cell.inertia.velocity = V2::zero();
        }
        //log!("index: {:?}, inertia: {new_inertia:?}", cell.index);
    }

    fn update_pos(&mut self, dt: f64) {
        // update grid and positions
        let mut grids_to_update = Vec::new();
        for (_cell_index, cell_ref) in &self.moving_cells {
            let mut cell = cell_ref.borrow_mut();
            let old_pos = cell.inertia.pos;
            let new_pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(dt));

            let new_pos_i = new_pos.round();
            // update grid:
            self.grids
                .update_cell_pos(&cell_ref, old_pos.round(), new_pos_i);
            // update position:
            cell.inertia.pos = new_pos;

            grids_to_update.push((self.grids.pos_to_index(new_pos_i), new_pos_i));
        }

        for (grid, pos) in grids_to_update {
            //  self.correct_positions(grid, pos, dt);
        }

        // Filter out moving cells that have been made static
        self.moving_cells.retain(|_, cell_ref| {
            let cell = cell_ref.borrow();
            cell.inertia.mass > 0
        });
    }

    fn correct_positions(&mut self, grid_index: GridIndex, pos: V2i, dt: f64) {
        // Apply position correction to prevent overlaps
        self.ensure_grid(grid_index);
        // ensure all surrounding grids are loaded
        self.ensure_grids(pos.plus(V2i::new(-1, -1)), pos.plus(V2i::new(1, 1)));

        let grid = self.grids.get(grid_index).unwrap();

        let get_res = grid.get(pos).clone();
        if get_res.value.is_empty() {
            return;
        }

        let mut moves = Vec::new();

        // Keep the first cell at this position
        let mut occupied_positions = FnvHashSet::default();
        occupied_positions.insert(pos);

        // If there are multiple cells in the same position,
        // find an empty nearby cell for all but one of them
        for cell_ref in get_res.value.iter().skip(1) {
            'outer: for nx in [0, -1, 1] {
                for ny in [-1, 1, 0] {
                    if nx == 0 && ny == 0 {
                        continue;
                    }
                    let npos = pos.plus(V2i::new(nx, ny));
                    let other_grid_index = self.grids.pos_to_index(npos);
                    let other_grid = self.grids.get(other_grid_index).unwrap();
                    if !other_grid.is_in_bounds(npos) {
                        continue;
                    }
                    if !other_grid.get(npos).value.is_empty() {
                        continue;
                    }
                    if occupied_positions.contains(&npos) {
                        continue;
                    }
                    occupied_positions.insert(npos);
                    moves.push((cell_ref.clone(), pos, npos));
                    break 'outer;
                }
            }
        }

        // Then apply all moves at once
        for (cell_ref, old_pos, new_pos) in moves {
            self.grids.update_cell_pos(&cell_ref, old_pos, new_pos);
            let mut cell = cell_ref.borrow_mut();
            cell.inertia.pos = new_pos.to_v2();
            if cell.inertia.velocity.magnitude_sqr() < velocity_threshold(dt) {
                // If the cell is moving slowly, make it static
                cell.set_static();
            } else {
                self.moving_cells.insert(cell.index, cell_ref.clone());
            }
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        let pos = cell.inertia.pos.round();
        let grid_index = self.grids.pos_to_index(pos);
        // don't allow adding too many cells in the same region
        let grid = self.grids.get_mut(grid_index).unwrap();
        let get_res = grid.get(pos);
        if get_res.neighbors.len() > 6 {
            return;
        }

        self.next_cell_index += 1;
        let index = CellIndex {
            index: self.next_cell_index,
        };
        let cell = Cell { index, ..cell };

        self.stats.cells_count += 1;
        let cell_ref = Rc::new(RefCell::new(cell));
        self.moving_cells.insert(index, cell_ref.clone());
        grid.put(pos, cell_ref.clone());
    }

    fn get_cells(&mut self, center: V2i, radius: usize) -> Vec<GridCellRef<Cell>> {
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
            let cell = cell_idx.borrow_mut();
            if cell.inertia.mass > 0 {
                continue;
            }
            self.moving_cells.insert(cell.index, cell_idx.clone());
            self.unstick_one_cell(cell);
        }
    }

    fn unstick_one_cell(&mut self, mut cell: std::cell::RefMut<'_, Cell>) {
        cell.unset_static();
        cell.inertia.velocity = V2 {
            x: 2.0 * ((cell.index.index as i32) % 10 - 5) as f64 / 10.0,
            y: -1.0 * ((cell.index.index as i32) % 10 - 5) as f64 / 10.0,
        };
    }

    pub fn remove_cell(&mut self, ppos: V2i) {
        let grid_index = self.grids.pos_to_index(ppos);
        self.ensure_grid(grid_index);

        let values: Vec<GridCellRef<Cell>> = self
            .grids
            .get(grid_index)
            .unwrap()
            .get(ppos)
            .value
            .iter()
            .map(|x| x.clone())
            .collect();

        for cell_ref in values {
            let cell = cell_ref.borrow();
            self.moving_cells.remove(&cell.index);
            self.grids
                .get_mut(grid_index)
                .unwrap()
                .remove(ppos, &cell_ref.clone());
        }
    }

    fn get_missing_grids(&self, center: V2) -> Vec<GridIndex> {
        let drop_radius = 2;
        self.grids.get_dropped_grids(center.round(), drop_radius)
    }

    fn get_loaded_grids(&self) -> Vec<GridIndex> {
        self.grids.get_loaded_grids()
    }

    fn get_droppable_grids(&self, center: V2) -> Vec<GridIndex> {
        let drop_radius = 2;
        self.grids.get_far_grids(center.round(), drop_radius)
    }

    fn save_grid(&mut self, grid_index: GridIndex) -> Option<&UniverseGrid<Cell>> {
        self.grids.get(grid_index)
    }

    pub fn drop_grid(&mut self, grid_index: GridIndex) {
        let maybe_grid = self.grids.get_mut(grid_index);
        let grid = match maybe_grid {
            Some(grid) => grid,
            None => {
                return;
            }
        };
        let grid_origin = grid_index.to_pos(grid.width, grid.height);
        for x in 0..grid.width {
            for y in 0..grid.height {
                let values = grid
                    .get(V2i::new(x as i32, y as i32).plus(grid_origin))
                    .value;
                for cell_index in values {
                    //self.cells.remove(&cell_index);
                    self.moving_cells.remove(&cell_index.borrow().index);
                }
            }
        }
        self.grids.drop_grid(grid_index);
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

    pub fn save_grid(&mut self, grid_index: GridIndex) -> Option<JsValue> {
        self.cells
            .save_grid(grid_index)
            .map(|grid| grid.to_bytes().unwrap())
    }

    pub fn drop_grid(&mut self, grid_index: GridIndex) {
        self.cells.drop_grid(grid_index)
    }

    pub fn get_missing_grids(&self) -> Vec<GridIndex> {
        self.cells.get_missing_grids(self.player.inertia.pos)
    }

    pub fn get_loaded_grids(&self) -> Vec<GridIndex> {
        self.cells.get_loaded_grids()
    }

    pub fn get_droppable_grids(&self) -> Vec<GridIndex> {
        self.cells.get_droppable_grids(self.player.inertia.pos)
    }

    pub fn load_from_storage(
        &mut self,
        grid_index: GridIndex,
        bytes: JsValue,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let grid = UniverseGrid::from_bytes(
            bytes,
            grid_index,
            self.cells.grids.grid_width,
            self.cells.grids.grid_height,
        )?;
        self.cells.load_from_storage(grid_index, grid);
        Ok(())
    }

    pub fn tick(&mut self) {
        self.cells.stats.ticks += 1;

        for _ in 0..((1.0 / self.dt) as usize) {
            //self.log_cells();

            self.calc_forces();
            self.update_velocity();

            self.cells.calc_collisions(self.dt);

            self.player.update_pos(&self.cells, self.dt);
            self.cells.update_pos(self.dt);
            self.zero_forces();
        }

        // player gets a bit sick as time passes
        self.player.life = self.player.life.saturating_sub(10000);

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
