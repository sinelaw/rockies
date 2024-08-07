use fnv::FnvHashMap;

use std::convert::TryFrom;

use crate::{grid::Grid, v2::V2i};

#[derive(Default, Hash, Eq, Clone, Copy, Debug, PartialEq)]
pub struct CellIndex {
    pub index: usize,
}

// Keeps track of the visible part of the world
pub struct UniverseGrid {
    pub width: usize,
    pub height: usize,

    offset: V2i,
    grid: Grid<CellIndex>,
}

impl UniverseGrid {
    pub fn is_in_bounds(&self, pos: V2i) -> bool {
        let relative_pos = pos.minus(self.offset);
        relative_pos.x >= 0
            && relative_pos.y >= 0
            && relative_pos.x < self.width as i32
            && relative_pos.y < self.height as i32
    }

    pub fn remove(&mut self, pos: V2i, cell_idx: CellIndex) {
        assert!(
            self.is_in_bounds(pos),
            "pos {pos:?} not in bounds, {:?}",
            self.offset
        );
        let rpos = pos.minus(self.offset);
        self.grid.remove(
            usize::try_from(rpos.x).unwrap(),
            usize::try_from(rpos.y).unwrap(),
            cell_idx,
        )
    }

    pub fn put(&mut self, pos: V2i, cell_idx: CellIndex) {
        assert!(self.is_in_bounds(pos));
        let rpos = pos.minus(self.offset);
        self.grid.put(
            usize::try_from(rpos.x).unwrap(),
            usize::try_from(rpos.y).unwrap(),
            cell_idx,
        )
    }

    pub fn get(&self, pos: V2i) -> crate::grid::GetResult<CellIndex> {
        assert!(self.is_in_bounds(pos));
        let rpos = pos.minus(self.offset);
        self.grid.get(
            usize::try_from(rpos.x).unwrap(),
            usize::try_from(rpos.y).unwrap(),
        )
    }

    pub(crate) fn new(grid_index: GridIndex, grid_width: usize, grid_height: usize) -> Self {
        UniverseGrid {
            grid: Grid::new(grid_width, grid_height),
            width: grid_width,
            height: grid_height,
            offset: grid_index.to_pos(grid_width, grid_height),
        }
    }
}

#[derive(Hash, Eq, Clone, Copy, Debug, PartialEq)]
pub struct GridIndex {
    // offset in units of width, height
    pub grid_offset: V2i,
}

impl GridIndex {
    pub fn from_pos(pos: V2i, width: usize, height: usize) -> GridIndex {
        GridIndex {
            grid_offset: V2i::new(
                pos.x.div_euclid(width as i32),
                pos.y.div_euclid(height as i32),
            ),
        }
    }

    pub fn to_pos(&self, width: usize, height: usize) -> V2i {
        V2i::new(
            self.grid_offset.x * width as i32,
            self.grid_offset.y * height as i32,
        )
    }
}

pub struct MultiGrid {
    grids: FnvHashMap<GridIndex, UniverseGrid>,

    pub grid_width: usize,
    pub grid_height: usize,
}

impl MultiGrid {
    pub fn new(width: usize, height: usize) -> MultiGrid {
        MultiGrid {
            grids: FnvHashMap::default(),

            grid_width: width,
            grid_height: height,
        }
    }

    pub fn or_insert_with(&mut self, index: GridIndex, f: impl Fn() -> UniverseGrid) -> bool {
        let is_new = !self.grids.contains_key(&index);
        self.grids.entry(index).or_insert_with(f);
        assert!(self.grids.contains_key(&index));
        is_new
    }

    pub fn get(&self, grid_index: GridIndex) -> Option<&UniverseGrid> {
        self.grids.get(&grid_index)
    }

    pub fn get_mut(&mut self, grid_index: GridIndex) -> Option<&mut UniverseGrid> {
        self.grids.get_mut(&grid_index)
    }

    fn remove(&mut self, grid_index: GridIndex) -> Option<UniverseGrid> {
        self.grids.remove(&grid_index)
    }

    pub fn pos_to_index(&self, pos: V2i) -> GridIndex {
        GridIndex::from_pos(pos, self.grid_width, self.grid_height)
    }

    pub fn update_cell_pos(&mut self, cell_idx: CellIndex, old_pos: V2i, new_pos: V2i) {
        // update grid:
        if old_pos != new_pos {
            self.get_mut(self.pos_to_index(old_pos))
                .map(|grid| grid.remove(old_pos, cell_idx));
            self.get_mut(self.pos_to_index(new_pos))
                .map(|grid| grid.put(new_pos, cell_idx));
        }
    }

    pub fn get_far_grids(&mut self, center: V2i, drop_radius: usize) -> Vec<GridIndex> {
        let grids_to_drop: Vec<GridIndex> = self
            .grids
            .iter()
            .filter(|(grid_index, _)| {
                let grid_pos = grid_index.to_pos(self.grid_width, self.grid_height);
                usize::try_from((grid_pos.x - center.x).abs()).unwrap() / self.grid_width
                    > drop_radius
                    || usize::try_from((grid_pos.y - center.y).abs()).unwrap() / self.grid_height
                        > drop_radius
            })
            .map(|(grid_index, _)| *grid_index)
            .collect();

        grids_to_drop
    }

    pub fn drop_grid(&mut self, grid_index: GridIndex) {
        //println!("dropping grid: {grid_index:?}");
        self.remove(grid_index);
    }
}

#[cfg(test)]
mod tests {
    use crate::multigrid::GridIndex;
    use crate::v2::V2i;

    #[test]
    fn test_grid_index_from_pos() {
        let width = 10;
        let height = 10;

        let pos = V2i::new(0, 0);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(0, 0));

        let pos = V2i::new(9, 9);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(0, 0));

        let pos = V2i::new(10, 10);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(1, 1));

        let pos = V2i::new(19, 19);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(1, 1));
    }

    #[test]
    fn test_grid_index_to_pos() {
        let width = 10;
        let height = 10;

        let index = GridIndex {
            grid_offset: V2i::new(0, 0),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(0, 0));

        let index = GridIndex {
            grid_offset: V2i::new(1, 1),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(10, 10));

        let index = GridIndex {
            grid_offset: V2i::new(1, 2),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(10, 20));

        let index = GridIndex {
            grid_offset: V2i::new(0, 2),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(0, 20));

        let index = GridIndex {
            grid_offset: V2i::new(-1, 2),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(-10, 20));
    }

    #[test]
    fn test_grid_index_from_pos_negative() {
        let width = 10;
        let height = 10;

        let pos = V2i::new(-1, -1);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(-1, -1));

        let pos = V2i::new(-11, -11);
        let index = GridIndex::from_pos(pos, width, height);
        assert_eq!(index.grid_offset, V2i::new(-2, -2));
    }

    #[test]
    fn test_grid_index_to_pos_negative() {
        let width = 10;
        let height = 10;

        let index = GridIndex {
            grid_offset: V2i::new(-1, -1),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(-10, -10));

        let index = GridIndex {
            grid_offset: V2i::new(-2, -2),
        };
        let pos = index.to_pos(width, height);
        assert_eq!(pos, V2i::new(-20, -20));
    }
}
