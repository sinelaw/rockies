use std::cell::RefCell;
use std::rc::Rc;

use noise::Vector2;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};

use crate::color::Color;
use crate::inertia::Inertia;
use crate::multigrid::{CellIndex, GridIndex, UniverseGrid};
use crate::universe::{Cell, UniverseCells};
use crate::v2::{V2i, V2};

pub struct Generator {
    hasher: PermutationTable,
}

impl Generator {
    pub fn new(seed: u32) -> Self {
        Self {
            hasher: PermutationTable::new(seed),
        }
    }

    fn generated_point(&self, pos: V2i) -> f64 {
        // Check for caverns
        let posv = pos.to_v2().cmul(0.01);

        // perlin_2d returns a value in (-1..1)
        let local_seed = perlin_2d(Vector2::new(posv.x, posv.y), &self.hasher).abs()
            * perlin_2d(Vector2::new(posv.y * 0.3, posv.x * 0.4), &self.hasher).abs();
        local_seed
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

    pub fn generate_pristine_grid(
        &mut self,
        grid_index: GridIndex,
        width: usize,
        height: usize,
    ) -> UniverseGrid<Cell> {
        let base_pos = grid_index.to_pos(width, height);
        let mut grid = UniverseGrid::new(grid_index, 64, 64);

        for x in 0..width {
            for y in 0..height {
                let pos = V2i::new(x as i32, y as i32).plus(base_pos);
                let altitude = height as i32 - pos.y;
                let above_ground = altitude > 0;
                if above_ground {
                    // generate "mountains"
                    let val = self.generated_point(V2i::new(pos.x, 0));

                    if val * 100.0 > altitude as f64 {
                        let cell = Self::wall_cell(pos, Color::hsv(30.0, 1.0, 0.5));
                        grid.put(pos, Rc::new(RefCell::new(cell)));
                    }
                } else {
                    // below ground
                    let val = self.generated_point(pos);
                    let depth = -altitude as f64;
                    if val < 0.02 + 0.5 / (depth * 0.1) {
                        let cell = Self::wall_cell(
                            pos,
                            Color::hsv(30.0, 1.0, (1.0 - val) * 0.5), // brown
                        );
                        grid.put(pos, Rc::new(RefCell::new(cell)));
                    }
                }
            }
        }
        grid
    }
}
