use crate::assets;
use crate::color::Color;
use crate::grid::Grid;
use crate::int_pair_set::IntPairSet;
use crate::v2::V2;
use wasm_bindgen::prelude::*;

extern crate web_sys;

const MAX_CELLS: usize = 4096;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        //    web_sys::console::log_1(&format!( $( $t )* ).into())
        //   println!( $( $t )* );
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inertia {
    pub velocity: V2,
    pub force: V2,
    pub pos: V2,
    pub mass: i32,
    pub elasticity: f64, // 0..1
    pub collision_stats: usize,
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
        self.inertia.pos = V2 {
            x: round(self.inertia.pos.x) as f64,
            y: round(self.inertia.pos.y) as f64,
        };
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
    w: usize,
    h: usize,
    inertia: Inertia,
    self_force: V2,
    frame: usize,
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
                pos: V2 {
                    x: round(x as f64) as f64,
                    y: round(y as f64) as f64,
                },
                mass: 10,
                elasticity: 0.5,
                collision_stats: 0,
            },
            self_force: V2::zero(),

            frame: 0,
        }
    }

    pub fn next_frame(&mut self) {
        self.frame += 1;
    }

    pub fn move_left(&mut self) {
        self.inertia.velocity.x -= 1.0;
        log!("force: {:?}", self.inertia.velocity);
    }

    pub fn move_right(&mut self) {
        self.inertia.velocity.x += 1.0;
        log!("force: {:?}", self.inertia.velocity);
    }

    pub fn move_up(&mut self) {
        self.inertia.velocity.y -= 1.0;
        log!("force: {:?}", self.inertia.velocity);
    }

    pub fn move_down(&mut self) {
        self.inertia.velocity.y += 1.0;
        log!("force: {:?}", self.inertia.velocity);
    }

    pub fn render(&self, pixels: &mut Vec<u32>, buf_width: usize, buf_height: usize) -> () {
        let hammy_0: (usize, usize, &[Color]) = assets::HAMMY_0;
        let hammy_1: (usize, usize, &[Color]) = assets::HAMMY_1;
        let hammy_2: (usize, usize, &[Color]) = assets::HAMMY_2;
        let hammies = [hammy_0, hammy_1, hammy_2];
        let (w, h, colors) = hammies[self.frame % 3];

        for x in 0..w {
            for y in 0..h {
                let py = (self.inertia.pos.y + y as f64);
                let px = (self.inertia.pos.x + x as f64);
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
}

pub struct Universe {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    moving_cells: Vec<CellIndex>,
    pub grid: Grid<CellIndex>,

    // transient data:
    collisions_list: Vec<(CellIndex, CellIndex)>,
    collisions_map: IntPairSet,

    gravity: V2,
    dt: f64,
    stats: Stats,

    pub player: Player,
}

fn inverse_mass(mass: i32) -> f64 {
    if mass == 0 {
        return 0.0;
    }
    return 1.0 / (mass as f64);
}

pub fn round(x: f64) -> i32 {
    (x + 0.5) as i32
}

impl Universe {
    fn calc_forces(&mut self) {
        self.player.inertia.force = self
            .player
            .inertia
            .force
            .plus(self.player.self_force)
            .plus(self.gravity.cmul(self.player.inertia.mass as f64));

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
        self.player.inertia.force = V2::zero();
        // self.player.self_force = V2::zero();
    }

    fn update_pos(&mut self) {
        log!("player pos: {:?}", self.player.inertia.pos);

        self.player.inertia.pos = self
            .player
            .inertia
            .pos
            .plus(self.player.inertia.velocity.cmul(self.dt));

        // some previously static cells may now need to be in moving_cells
        self.moving_cells.clear();
        for cell in &self.cells {
            if !self.is_in_bounds(cell.inertia.pos) {
                continue;
            }
            //} && (cell.inertia.velocity.len() > self.velocity_threshold())
            if cell.inertia.mass > 0 {
                self.moving_cells.push(cell.index);
            }
        }
        // update grid and positions
        for cell_index in &self.moving_cells {
            let cell = &self.cells[cell_index.index];
            let old_pos = cell.inertia.pos;
            let new_pos = cell.inertia.pos.plus(cell.inertia.velocity.cmul(self.dt));

            // update grid:
            if self.is_in_bounds(old_pos) {
                self.grid
                    .remove(old_pos.x as usize, old_pos.y as usize, cell.index)
            }
            if self.is_in_bounds(new_pos) {
                self.grid
                    .put(new_pos.x as usize, new_pos.y as usize, cell.index);
            }
            // update position:
            self.cells[cell_index.index].inertia.pos = new_pos;
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
    fn clamp_velocity(v: V2) -> V2 {
        let max = V2 {
            x: 0.5 / 0.1,
            y: 0.5 / 0.1,
        };
        let min = V2 {
            x: -0.5 / 0.1,
            y: -0.5 / 0.1,
        };
        return v.min(max).max(min);
    }

    fn update_vel(&mut self) {
        self.player.inertia.velocity = Self::clamp_velocity(
            self.player.inertia.velocity.plus(
                self.player
                    .inertia
                    .force
                    .cdiv(self.player.inertia.mass as f64)
                    .cmul(self.dt),
            ),
        );

        //        log!("update_vel: player: {:?}", self.player.inertia);

        for cell in &mut self.cells {
            if cell.inertia.mass > 0 {
                cell.inertia.velocity = Self::clamp_velocity(
                    cell.inertia.velocity.plus(
                        cell.inertia
                            .force
                            .cdiv(cell.inertia.mass as f64)
                            .cmul(self.dt),
                    ),
                );
            }
        }
    }

    fn is_in_bounds(&self, pos: V2) -> bool {
        pos.x >= 0.0 && pos.x < self.width as f64 && pos.y >= 0.0 && pos.y < self.height as f64
    }

    fn velocity_threshold(&self) -> f64 {
        self.dt / 10.0
    }

    fn collect_collisions(&mut self) {
        self.collisions_map.clear();
        self.collisions_list.clear();

        for cell1_idx in self.moving_cells.iter() {
            let cell1 = &self.cells[cell1_idx.index];

            if !self.is_in_bounds(cell1.inertia.pos) {
                continue;
            }
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

                // log!("cell1: {cell1:?}");
                // log!("cell2: {cell2:?}");

                // println!("collisions: {:?}", collisions);
            }
        }
    }

    fn calc_collisions(&mut self) {
        self.collect_collisions();
        self.stats.collisions_count += self.collisions_list.len();
        for (cell1_idx, cell2_idx) in self.collisions_list.iter() {
            let inertia2 = &self.cells[cell2_idx.index].inertia;
            let inertia1 = &self.cells[cell1_idx.index].inertia;

            // static cell is involved, make them both static
            if ((inertia1.mass == 0) || (inertia2.mass == 0))
                && (inertia1.velocity.magnitude_sqr() < self.velocity_threshold())
                && (inertia2.velocity.magnitude_sqr() < self.velocity_threshold())
            {
                self.cells[cell1_idx.index].set_static();
                self.cells[cell2_idx.index].set_static();

                continue;
            }

            let (new_vel1, new_vel2) = collide(inertia1, inertia2);

            self.cells[cell1_idx.index].inertia.velocity = new_vel1;
            self.cells[cell2_idx.index].inertia.velocity = new_vel2;
        }
    }

    fn collide_player(&mut self) {
        let px = self.player.inertia.pos.x + self.player.w as f64 / 2.0;
        let py = self.player.inertia.pos.y + self.player.h as f64 / 2.0;

        if !self.is_in_bounds(V2 { x: px, y: py }) {
            return;
        };
        let (neighbors_count, neighbors) = self.grid.get(px as usize, py as usize);
        if neighbors_count == 0 {
            return;
        }
        let mut new_vel = V2::zero();
        for cell_idx in neighbors.iter() {
            let cell = &self.cells[cell_idx.index];
            let cell_mass = cell.inertia.mass;
            let player_inertia = Inertia {
                pos: V2 { x: px, y: py },
                ..self.player.inertia
            };
            let (cell_vel, player_vel) = collide(&cell.inertia, &player_inertia);
            log!(
                "collide cell_vel: {:?}, cell before: {:?}",
                cell_vel,
                cell.inertia
            );
            self.cells[cell_idx.index].inertia.velocity = cell_vel;
            new_vel = new_vel.plus(player_vel);

            log!("collide player: {:?}", player_inertia);
            log!("new_vel: {:?}", new_vel);
            log!("player_vel: {:?}", player_vel);
            log!("pos player: {:?}", (px, py));
            break;
        }
        self.player.inertia.velocity = Self::clamp_velocity(new_vel);

        log!("final player: {:?}", self.player.inertia);
        log!("px,py: {px},{py} neighbors: {neighbors_count}");
        log!("--------------------");

        //log!("final player: {:?}", self.player.inertia);
    }

    pub fn tick(&mut self) {
        self.stats.ticks += 1;
        self.reset_cells();

        for _ in 0..((1.0 / self.dt) as usize) {
            //self.log_cells();

            self.calc_forces();
            self.update_vel();

            self.calc_collisions();
            self.collide_player();
            self.update_pos();
            self.zero_forces();
        }

        //log!("{}", self.render());
    }

    pub fn add_cell(&mut self, cell: Cell) {
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
        self.cells.push(Cell { index, ..cell });
        self.grid.put(
            cell.inertia.pos.x as usize,
            cell.inertia.pos.y as usize,
            index,
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
                elasticity: 1.0, // allow other mass to determine
                collision_stats: 0,
            },
        }
    }
    pub fn new(width: usize, height: usize) -> Universe {
        let mut uni = Universe {
            width: width,
            height: height,
            cells: Vec::new(),
            moving_cells: Vec::new(),
            grid: Grid::new(width as usize, height as usize),

            gravity: V2 { x: 0.0, y: 0.1 },
            dt: 0.01,
            stats: Stats::zero(),

            collisions_list: Vec::new(),
            collisions_map: IntPairSet::new(MAX_CELLS),

            player: Player::new(10, 10),
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

    fn reset_cells(&mut self) {
        for cell in &mut self.cells {
            cell.inertia.collision_stats = 0;
        }
    }

    pub fn stats(&mut self) -> Stats {
        let res = self.stats;
        self.stats = Stats::zero();
        res
    }

    pub fn unstick_cells(&mut self, x: usize, y: usize, radius: usize) {
        let r = radius as i32;
        for i in -r..r {
            for j in -r..r {
                let (px, py) = (x as i32 + i, y as i32 + j);
                if !self.is_in_bounds(V2 {
                    x: px as f64,
                    y: py as f64,
                }) {
                    continue;
                }
                let (neighbors_count, neighbors) = self.grid.get(px as usize, py as usize);
                let w = self.width as f64;
                for cell_idx in &neighbors[0..neighbors_count] {
                    let cell = &mut self.cells[cell_idx.index];
                    if cell.inertia.mass > 0 {
                        continue;
                    }
                    cell.unset_static();
                    self.moving_cells.push(*cell_idx);
                    cell.inertia.velocity = V2 {
                        x: 2.0 * (x as f64 - w / 2.0) / w,
                        y: -1.0, //(cell_idx.index % 10 - 5) as f64 / 10000.0 * self.dt,
                    };
                }
            }
        }
    }
}

fn collide(inertia1: &Inertia, inertia2: &Inertia) -> (V2, V2) {
    let rel_velocity = inertia1.velocity.minus(inertia2.velocity);
    let normal = inertia1.pos.minus(inertia2.pos);
    // coefficient of restitution
    let e = inertia1.elasticity.min(inertia2.elasticity);

    // let collision_vel = rel_velocity.dot(normal);
    let collision_vel: f64 = rel_velocity.dot(normal) as f64 * -(1.0 + e);

    // for simplicity the rest here treats them as circles, not boxes:
    let distance = normal.magnitude();

    let normal_direction = if distance == 0.0 {
        // the two are perfectly aligned on top of each other
        V2 { x: 1.0, y: 0.0 }
    } else {
        normal.cdiv(distance)
    };

    let im1 = inverse_mass(inertia1.mass);
    let im2 = inverse_mass(inertia2.mass);

    let impulse = collision_vel / (im1 + im2);

    /*  log!("rel_velocity: {rel_velocity:?}");
    log!("norm: {normal:?}");
    log!("collision_vel: {collision_vel:?}"); */

    (
        inertia1.velocity.plus(normal_direction.cmul(impulse * im1)),
        inertia2
            .velocity
            .minus(normal_direction.cmul(impulse * im2)),
    )
}
