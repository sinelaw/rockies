use std::collections::HashSet;
use std::sync::LazyLock;
mod inertia;

mod assets;
mod color;
mod grid;
mod multigrid;
mod universe;
mod utils;
mod v2;
use color::Color;

mod log;

use noise::Vector2;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};

use inertia::Inertia;
use log::log;
use multigrid::{CellIndex, GridIndex};
use serde::Deserialize;
use universe::{Cell, Stats, Universe};

use v2::{V2i, V2};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    universe: Universe,
    keys: HashSet<String>,
    shoot_color: Color,
    hasher: PermutationTable,
}

static GRID_SIZE: usize = 128;

static BUILD_TIME: LazyLock<chrono::DateTime<chrono::Utc>> = LazyLock::new(|| chrono::Utc::now());

macro_rules! cargo_build_time {
    () => {
        BUILD_TIME.to_rfc3339()
    };
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        utils::set_panic_hook();

        Self {
            width,
            height,
            pixels: vec![0xFFFFFF; (width * height) as usize],
            universe: Universe::new(GRID_SIZE, GRID_SIZE),
            keys: HashSet::new(),
            shoot_color: Color::hsv(90.0, 1.0, 1.0),
            hasher: PermutationTable::new(1),
        }
    }

    pub fn pixels(&self) -> *const u32 {
        self.pixels.as_ptr()
    }

    pub fn version(&self) -> String {
        cargo_build_time!()
    }

    pub fn tick(&mut self) {
        self.render();
        self.process_keys();
        self.universe.tick();
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < (self.width as i32) && y >= 0 && y < (self.height as i32)
    }

    pub fn get_grids_to_load(&self) -> Vec<GridIndex> {
        self.universe.get_grids_to_load()
    }

    pub fn get_grids_to_save(&self) -> Vec<GridIndex> {
        self.universe.get_grids_to_save()
    }

    pub fn load_grid(&mut self, grid_index: &GridIndex, bytes: JsValue) {
        if let Err(err) = self.universe.load_from_storage(*grid_index, bytes) {
            log!("Failed to load grid {grid_index:?}: {}", err);
        }
    }

    pub fn generate_grid(&mut self, grid_index: &GridIndex) {
        self.universe.cells.ensure_grid(*grid_index);
    }

    pub fn save_grid(&mut self, grid_index: &GridIndex) -> JsValue {
        if let Some(bytes) = self.universe.drop_to_storage(*grid_index) {
            bytes
        } else {
            JsValue::null()
        }
    }

    pub fn render(&mut self) -> () {
        let is_dig_mode = self.is_dig_mode();
        self.pixels.fill(0xFFFFFF);

        let w = self.width as i32;
        let h = self.height as i32;
        let render_offset = V2i::new(w / 2, h / 2);
        let base_pos = self
            .universe
            .player
            .inertia
            .pos
            .round()
            .minus(render_offset);
        let get_res = self
            .universe
            .cells
            .get_range(base_pos, base_pos.plus(V2i::new(w, h)));
        for res in get_res.iter() {
            self.render_cell(res, base_pos);
        }
        self.universe.player.render(
            &mut self.pixels,
            self.universe.player.inertia.pos.round().minus(base_pos),
            self.width,
            self.height,
            is_dig_mode,
        );
    }

    fn render_cell(&mut self, res: &(V2i, Vec<grid::GridCellRef<Cell>>), base_pos: V2i) {
        let pos = res.0;
        let pixel_pos = pos.minus(base_pos);
        let w = self.width as i32;
        let pixel_idx = (pixel_pos.y * w + pixel_pos.x) as usize;
        if res.1.is_empty() {
            self.pixels[pixel_idx] = self.render_background(pos);
            return;
        }
        self.pixels[pixel_idx] = 0;
        for cell_ref in res.1.iter() {
            let cell = cell_ref.borrow();
            let cell_color = if cell.inertia.collision_stats > 0 && cell.inertia.mass > 0 {
                0xFF0000
            } else {
                cell.color.to_u32()
            };
            self.pixels[pixel_idx] = self.pixels[pixel_idx].saturating_add(cell_color);
        }
    }

    fn render_background(&self, pos: V2i) -> u32 {
        let hasher = &self.hasher;
        let depth = pos.y - (self.height as i32);
        if depth >= self.height as i32 {
            // underground - deeper is darker
            let value = (255.0 / ((depth + 2) as f64).powf(0.5)) as u32;
            value + (value << 8) + (value << 16)
        } else {
            let altitude = -depth as f64 + self.height as f64;
            // generate clouds
            let posv = pos.to_v2().plus(V2::new(0.5, 0.7)).cmul(0.01);
            let noise2 = perlin_2d(Vector2::new(posv.y * 10.0, posv.x * 10.0), hasher);
            let noise = perlin_2d(Vector2::new(posv.x, posv.y), hasher);
            if (0.2 + 0.9 / (altitude / 10.0)) < noise2 * noise {
                0xFFFFFF
            } else {
                0xCCCCFF
            }
        }
    }

    pub fn key_down(&mut self, key: String) {
        self.keys.insert(key.to_ascii_lowercase());
    }

    pub fn key_up(&mut self, key: String) {
        self.keys.remove(&key.to_ascii_lowercase());
    }

    pub fn unfocus(&mut self) {
        self.keys.clear();
    }

    fn is_dig_mode(&self) -> bool {
        self.keys.iter().any(|k| k == "shift")
    }

    pub fn process_keys(&mut self) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();

        // shift is down => dig mode
        let is_dig_mode = self.is_dig_mode();

        for raw_key in self.keys.iter() {
            if raw_key.len() > 1 {
                continue;
            }
            // single-char keys:
            let key = raw_key.chars().nth(0).unwrap();
            match key {
                c @ '0'..='9' => {
                    self.shoot_color =
                        Color::hsv((c as u8 - '0' as u8) as f64 / 10.0 * 360.0, 1.0, 1.0);
                }
                'a' => {
                    self.universe.player.move_left();
                    xs.push(-1);
                    xs.push(-2);
                }
                'd' => {
                    self.universe.player.move_right();
                    xs.push(self.universe.player.w as i32);
                    xs.push((self.universe.player.w + 1) as i32);
                }
                'w' => {
                    self.universe.player.move_up();
                    ys.push(-1);
                    ys.push(-2);
                }
                's' => {
                    self.universe.player.move_down();
                    ys.push(self.universe.player.h as i32);
                    ys.push((self.universe.player.h + 1) as i32);
                }

                ' ' => {
                    self.universe.player.next_frame();

                    self.universe.cells.add_cell(Cell {
                        index: CellIndex { index: 0 },
                        color: self.shoot_color,
                        inertia: Inertia {
                            velocity: V2::new(1.0 * (self.universe.player.direction as f64), -1.0),
                            force: V2::zero(),
                            pos: self.universe.player.mouth_pos(),
                            mass: 1,
                            elasticity: universe::ELASTICITY,
                            collision_stats: 0,
                        },
                    });
                }
                _ => (),
            }
        }
        if is_dig_mode {
            let pos: V2i = self.universe.player.inertia.pos.round();
            for x in 0..self.universe.player.w {
                for y in 0..self.universe.player.h {
                    self.universe
                        .cells
                        .remove_cell(pos.plus(V2i::new(x as i32, y as i32)));
                }
            }
            for x in xs.iter() {
                for y in ys.iter() {
                    self.universe.cells.remove_cell(pos.plus(V2i::new(*x, *y)));
                }
            }
            for x in 0..self.universe.player.w {
                for y in ys.iter() {
                    self.universe
                        .cells
                        .remove_cell(pos.plus(V2i::new(x as i32, *y)));
                }
            }
            for y in 0..self.universe.player.h {
                for x in xs.iter() {
                    self.universe
                        .cells
                        .remove_cell(pos.plus(V2i::new(*x, y as i32)));
                }
            }
        }
    }

    pub fn click(&mut self, x: i32, y: i32) {
        if !self.is_in_bounds(x, y) {
            return;
        }
        let w = self.width as i32;
        let h = self.height as i32;
        let render_offset = V2i::new(w / 2, h / 2);
        let base_pos = self
            .universe
            .player
            .inertia
            .pos
            .round()
            .minus(render_offset);
        let pos = base_pos.plus(V2i::new(x, y));
        // unstick some cells
        self.universe.cells.unstick_cells(pos, 3);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn stats(&mut self) -> Stats {
        self.universe.stats()
    }
}

impl Game {
    pub fn pixels_vec(&self) -> &Vec<u32> {
        &self.pixels
    }
}
