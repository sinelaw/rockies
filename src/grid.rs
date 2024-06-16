use std::hash::Hash;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    grid: Vec<Vec<T>>,
    empty_vec: Vec<T>,
}

const FACTOR: usize = 1;

fn grid_index(x: usize, y: usize, height: usize) -> usize {
    (x / FACTOR) * (height / FACTOR + 2) + (y / FACTOR)
}

impl<T: Hash + Clone + Eq> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid: Vec<Vec<T>> = Vec::new();
        grid.resize(
            ((width / FACTOR + 2) * (height / FACTOR + 2)) as usize,
            Vec::with_capacity(9 * 9),
        );
        Grid {
            width,
            height,
            grid,
            empty_vec: Vec::new(),
        }
    }

    pub fn put(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.width);
        assert!(y < self.height);
        for px in 0..3 {
            for py in 0..3 {
                self.grid[grid_index(x + px, y + py, self.height)].push(value.clone());
            }
        }
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);
        for px in 0..3 {
            for py in 0..3 {
                self.grid[grid_index(x + px, y + py, self.height)].clear();
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> [[&Vec<T>; 3]; 3] {
        assert!(x < self.width);
        assert!(y < self.height);

        let mut res: [[&Vec<T>; 3]; 3] = [
            [&self.empty_vec, &self.empty_vec, &self.empty_vec],
            [&self.empty_vec, &self.empty_vec, &self.empty_vec],
            [&self.empty_vec, &self.empty_vec, &self.empty_vec],
        ];
        for px in 0..3 {
            for py in 0..3 {
                res[px][py] = &self.grid[grid_index(x + px, y + py, self.height)];
            }
        }
        res
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
        let res = grid.get(0, 0);

        println!("{:?}", grid);

        let expected = [
            [&vec!['a'], &vec!['a'], &vec!['a']],
            [&vec!['a'], &vec!['a'], &vec!['a']],
            [&vec!['a'], &vec!['a'], &vec!['a']],
        ];

        assert_eq!(res, expected);
    }

    #[test]
    fn test_grid_two() {
        let mut grid: Grid<char> = Grid::new(2, 1);
        grid.put(0, 0, 'a');
        grid.put(1, 0, 'b');

        let res = grid.get(0, 0);

        let expected = [
            [&vec!['a'], &vec!['a'], &vec!['a']],
            [&vec!['a', 'b'], &vec!['a', 'b'], &vec!['a', 'b']],
            [&vec!['a', 'b'], &vec!['a', 'b'], &vec!['a', 'b']],
        ];

        assert_eq!(res, expected);
    }
}
