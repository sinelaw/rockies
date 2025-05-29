// The `Grid` struct provides a 2D grid data structure for storing and retrieving items.
// It is designed to efficiently handle spatial queries, particularly for finding items
// within a certain proximity (neighbors) of a given location.
//
// High-Level Concepts:
// - **GridCell**: Represents a single cell in the grid. Each `GridCell` can hold multiple
//   items (`value`) and maintain a list of items in neighboring cells (`neighbors`).
// - **Neighbors**: The `Grid` pre-calculates and stores references to items in adjacent
//   cells within each `GridCell`. This allows for fast retrieval of nearby items without
//   iterating over the entire grid. This is particularly useful for collision detection
//   or other proximity-based operations.
// - **Versioning**: The `GridCell` uses a `version` to track changes. This allows for
//   efficient clearing of cell data without reallocating memory.
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use wasm_bindgen::JsValue;

pub type GridCellRef<T> = Rc<RefCell<T>>;

#[derive(Serialize, Deserialize)]
struct GridSerialData<T> {
    width: usize,
    height: usize,
    version: usize,
    items: Vec<(usize, usize, T)>, // (x, y, item)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetResult<'a, T> {
    pub value: &'a [GridCellRef<T>],
    pub neighbors: &'a [GridCellRef<T>],
}

#[derive(Debug, Clone)]
struct GridCell<T> {
    version: usize,
    value: Vec<GridCellRef<T>>,
    neighbors: Vec<GridCellRef<T>>, // [T; 16],
}

impl<T: Debug> GridCell<T> {
    pub fn new() -> GridCell<T> {
        GridCell {
            version: 0,
            value: Vec::with_capacity(4),
            neighbors: Vec::with_capacity(16), //[T::default(); 16],
        }
    }

    pub fn get(&self, version: usize) -> GetResult<T> {
        if version == self.version {
            GetResult {
                value: &self.value,
                neighbors: &self.neighbors,
            }
        } else {
            GetResult {
                value: &self.value[0..0],
                neighbors: &self.neighbors[0..0],
            }
        }
    }

    fn ensure_version(&mut self, version: usize) {
        if version != self.version {
            self.version = version;
            self.value.clear();
            self.neighbors.clear();
        }
    }

    pub fn set_value(&mut self, version: usize, value: GridCellRef<T>) {
        self.ensure_version(version);
        self.value.push(value);
    }

    pub fn remove_value(&mut self, version: usize, value: &GridCellRef<T>) {
        if version != self.version {
            return;
        }
        self.value.retain(|x| !Rc::ptr_eq(x, value));
    }

    pub fn add_neighbor(&mut self, version: usize, neighbor: GridCellRef<T>) {
        self.ensure_version(version);
        self.neighbors.push(neighbor);
    }

    pub fn remove_neighbor(&mut self, version: usize, neighbor: &GridCellRef<T>) {
        if version != self.version {
            return;
        }
        self.neighbors.retain(|x| !Rc::ptr_eq(x, neighbor));
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<GridCell<T>>,
    version: usize,
}

const FACTOR: usize = 1;

fn grid_index(x: usize, y: usize, height: usize) -> usize {
    (x / FACTOR) * (height / FACTOR + 2) + (y / FACTOR)
}

/// Data organized in 2d
impl<T: Debug> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid: Vec<GridCell<T>> =
            Vec::with_capacity(((width / FACTOR + 2) * (height / FACTOR + 2)) as usize);
        for _ in 0..((width / FACTOR + 2) * (height / FACTOR + 2)) {
            grid.push(GridCell::new());
        }
        Grid {
            width,
            height,
            grid,
            version: 0,
        }
    }

    pub fn put(&mut self, x: usize, y: usize, value: GridCellRef<T>) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.grid[grid_index(x + 1, y + 1, self.height)].set_value(self.version, value.clone());
        for px in 0..3 {
            for py in 0..3 {
                self.grid[grid_index(x + px, y + py, self.height)]
                    .add_neighbor(self.version, value.clone());
            }
        }
    }
    /*
    pub fn clear(&mut self) {
        self.version += 1;
    } */

    pub fn get(&self, x: usize, y: usize) -> GetResult<T> {
        assert!(x < self.width);
        assert!(y < self.height);
        self.grid[grid_index(x + 1, y + 1, self.height)].get(self.version)
    }

    pub fn remove(&mut self, x: usize, y: usize, value: &GridCellRef<T>) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.grid[grid_index(x + 1, y + 1, self.height)].remove_value(self.version, value);
        for px in 0..3 {
            for py in 0..3 {
                self.grid[grid_index(x + px, y + py, self.height)]
                    .remove_neighbor(self.version, value);
            }
        }
    }
}

impl<T: Debug + Clone> Grid<T> {
    /// Serialize the grid to bytes
    /// This serializes the grid dimensions and all items with their positions
    pub fn to_bytes(&self) -> Result<JsValue, serde_wasm_bindgen::Error>
    where
        T: serde::Serialize,
    {
        let mut items = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                let result = self.get(x, y);
                for item_ref in result.value {
                    let item = item_ref.borrow().clone();
                    items.push((x, y, item));
                }
            }
        }

        let grid_data = GridSerialData {
            width: self.width,
            height: self.height,
            version: self.version,
            items,
        };

        serde_wasm_bindgen::to_value(&grid_data)
    }

    /// Deserialize the grid from bytes
    pub fn from_bytes(bytes: JsValue) -> Result<Self, serde_wasm_bindgen::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let grid_data: GridSerialData<T> = serde_wasm_bindgen::from_value(bytes)?;

        let mut grid = Grid::new(grid_data.width, grid_data.height);
        grid.version = grid_data.version;

        // Reconstruct the grid by placing items at their positions
        for (x, y, item) in grid_data.items {
            let item_ref = Rc::new(RefCell::new(item));
            grid.put(x, y, item_ref);
        }

        Ok(grid)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_grid_new_empty() {
        let _grid: Grid<i32> = Grid::new(0, 0);
    }

    #[test]
    fn test_grid_one() {
        let mut grid: Grid<char> = Grid::new(1, 1);
        let a = Rc::new(RefCell::new('a'));
        grid.put(0, 0, a.clone());
        let res = grid.get(0, 0);
        assert_eq!(res.neighbors.len(), 1);
        assert_eq!(res.value, &[a.clone()]);
        assert_eq!(res.neighbors, &[a.clone()]);

        grid.remove(0, 0, &a);
        let res = grid.get(0, 0);
        assert_eq!(res.neighbors.len(), 0);
        assert_eq!(res.value, &[]);
    }

    #[test]
    fn test_grid_two() {
        let mut grid: Grid<char> = Grid::new(2, 1);
        let a = Rc::new(RefCell::new('a'));
        let b = Rc::new(RefCell::new('b'));
        grid.put(0, 0, a.clone());
        grid.put(1, 0, b.clone());

        let res = grid.get(0, 0);

        assert_eq!(res.neighbors.len(), 2);
        assert_eq!(res.value, &[a.clone()]);
        assert_eq!(res.neighbors, &[a.clone(), b.clone()]);

        grid.remove(0, 0, &a);
        let res = grid.get(0, 0);
        assert_eq!(res.neighbors.len(), 1);
        assert_eq!(res.value, &[]);
        assert_eq!(res.neighbors, &[b.clone()]);
    }

    #[test]
    fn test_grid_two_apart() {
        let mut grid: Grid<char> = Grid::new(6, 2);
        let a = Rc::new(RefCell::new('a'));
        let b = Rc::new(RefCell::new('b'));
        grid.put(0, 0, a.clone());
        grid.put(4, 0, b.clone());

        {
            let res = grid.get(0, 0);
            assert_eq!(res.neighbors.len(), 1);
            assert_eq!(res.value, &[a.clone()]);
            assert_eq!(res.neighbors, &[a.clone()]);
        }
        {
            let res = grid.get(4, 0);
            assert_eq!(res.neighbors.len(), 1);
            assert_eq!(res.value, &[b.clone()]);
            assert_eq!(res.neighbors, &[b.clone()]);
        }
    }

    #[test]
    fn test_grid_serialization() {
        let mut grid: Grid<char> = Grid::new(3, 5);
        let a = Rc::new(RefCell::new('a'));
        let b = Rc::new(RefCell::new('b'));
        let c = Rc::new(RefCell::new('c'));

        grid.put(0, 0, a.clone());
        grid.put(1, 1, b.clone());
        grid.put(2, 2, c.clone());

        // Serialize the grid
        let bytes = grid.to_bytes();
        assert!(!bytes.is_err());

        // Deserialize the grid
        let restored_grid: Grid<char> = Grid::from_bytes(bytes.unwrap()).unwrap();

        // Verify dimensions and version
        assert_eq!(restored_grid.width, 3);
        assert_eq!(restored_grid.height, 5);
        assert_eq!(restored_grid.version, grid.version);

        // Verify items are in correct positions
        let res_a = restored_grid.get(0, 0);
        assert_eq!(res_a.value.len(), 1);
        assert_eq!(*res_a.value[0].borrow(), 'a');

        let res_b = restored_grid.get(1, 1);
        assert_eq!(res_b.value.len(), 1);
        assert_eq!(*res_b.value[0].borrow(), 'b');

        let res_c = restored_grid.get(2, 2);
        assert_eq!(res_c.value.len(), 1);
        assert_eq!(*res_c.value[0].borrow(), 'c');

        // Verify empty positions
        let res_empty = restored_grid.get(0, 1);
        assert_eq!(res_empty.value.len(), 0);
    }

    #[test]
    fn test_grid_empty_serialization() {
        let grid: Grid<i32> = Grid::new(2, 2);

        let bytes = grid.to_bytes();
        assert!(!bytes.is_err());

        let restored_grid: Grid<i32> = Grid::from_bytes(bytes.unwrap()).unwrap();
        assert_eq!(restored_grid.width, 2);
        assert_eq!(restored_grid.height, 2);

        // All positions should be empty
        for x in 0..2 {
            for y in 0..2 {
                let res = restored_grid.get(x, y);
                assert_eq!(res.value.len(), 0);
            }
        }
    }

    /*
    #[test]
    fn test_grid_clear() {
        let mut grid: Grid<char> = Grid::new(2, 2);
        grid.put(0, 0, 'a');

        grid.clear();
        let (count, _) = grid.get(0, 0);
        assert_eq!(count, 0);

        grid.put(1, 1, 'b');

        let (count, items) = grid.get(1, 1);
        assert_eq!(count, 1);
        assert_eq!(items[0..count], ['b'][..]);
    } */
}
