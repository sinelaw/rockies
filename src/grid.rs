use std::collections::HashSet;
use std::hash::Hash;

pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<Vec<Vec<T>>>,
}

impl<T: Hash + Clone + Eq> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid: Vec<Vec<Vec<T>>> = Vec::new();
        grid.resize((width + 2) as usize, {
            let mut v = Vec::new();
            v.resize((height + 2) as usize, Vec::new());
            v
        });
        Grid {
            width,
            height,
            grid,
        }
    }

    pub fn put(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        for px in 0..3 {
            for py in 0..3 {
                self.grid[x + px][y + py].push(value.clone());
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> HashSet<T> {
        assert!(x < self.width);
        assert!(y < self.height);
        let mut res = HashSet::new();
        for px in 0..3 {
            for py in 0..3 {
                for v in self.grid[x + px][y + py].iter() {
                    res.insert(v.clone());
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_grid_new_empty() {
        let _grid: Grid<i32> = Grid::new(0, 0);
    }

    #[test]
    fn test_grid_one() {
        let mut grid: Grid<char> = Grid::new(1, 1);
        grid.put(0, 0, 'a');
        let res = grid.get(0, 0);

        let mut expected = HashSet::new();
        expected.insert('a');

        assert_eq!(res, expected);
    }

    #[test]
    fn test_grid_two() {
        let mut grid: Grid<char> = Grid::new(2, 1);
        grid.put(0, 0, 'a');
        grid.put(1, 0, 'b');
        let res = grid.get(0, 0);

        let mut expected = HashSet::new();
        expected.insert('a');
        expected.insert('b');

        assert_eq!(res, expected);
    }
}
