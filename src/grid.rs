use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GridCell<T> {
    count: usize,
    version: usize,
    items: [T; 16],
}

impl<T: Default + Copy> GridCell<T> {
    pub fn new() -> GridCell<T> {
        GridCell {
            count: 0,
            version: 0,
            items: [T::default(); 16],
        }
    }

    pub fn get(&self, version: usize) -> (usize, &[T]) {
        let count = if version != self.version {
            0
        } else {
            self.count
        };
        (count, &self.items)
    }

    pub fn push(&mut self, version: usize, item: T) {
        if version != self.version {
            self.count = 0;
            self.version = version;
        }
        assert!(self.count < self.items.len());
        self.items[self.count] = item;
        self.count += 1;
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
impl<T: Default + Copy + Hash + Clone + Eq> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid: Vec<GridCell<T>> = Vec::new();
        grid.resize(
            ((width / FACTOR + 2) * (height / FACTOR + 2)) as usize,
            GridCell::new(),
        );
        Grid {
            width,
            height,
            grid,
            version: 0,
        }
    }

    pub fn put(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        for px in 0..3 {
            for py in 0..3 {
                self.grid[grid_index(x + px, y + py, self.height)]
                    .push(self.version, value.clone());
            }
        }
    }

    pub fn clear(&mut self) {
        self.version += 1;
    }

    pub fn get(&self, x: usize, y: usize) -> (usize, &[T]) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.grid[grid_index(x + 1, y + 1, self.height)].get(self.version)
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
        grid.put(0, 0, 'a');
        let (count, items) = grid.get(0, 0);
        assert_eq!(count, 1);
        assert_eq!(items[0..count], ['a'][..]);
    }

    #[test]
    fn test_grid_two() {
        let mut grid: Grid<char> = Grid::new(2, 1);
        grid.put(0, 0, 'a');
        grid.put(1, 0, 'b');

        let (count, items) = grid.get(0, 0);

        assert_eq!(count, 2);
        assert_eq!(items[0..count], ['a', 'b'][..]);
    }

    #[test]
    fn test_grid_two_apart() {
        let mut grid: Grid<char> = Grid::new(6, 2);
        grid.put(0, 0, 'a');
        grid.put(4, 0, 'b');

        {
            let (count, items) = grid.get(0, 0);
            assert_eq!(count, 1);
            assert_eq!(items[0..count], ['a'][..]);
        }
        {
            let (count, items) = grid.get(4, 0);
            assert_eq!(count, 1);
            assert_eq!(items[0..count], ['b'][..]);
        }
    }

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
    }
}
