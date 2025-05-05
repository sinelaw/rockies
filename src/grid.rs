use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type GridCellRef<T> = Rc<RefCell<T>>;

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
