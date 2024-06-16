pub struct Grid<T> {
    grid: Vec<Vec<Vec<T>>>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let mut grid: Vec<Vec<Vec<T>>> = Vec::new();
        grid.resize((width + 2) as usize, {
            let mut v = Vec::new();
            v.resize((height + 2) as usize, Vec::new());
            v
        });
        Grid { grid }
    }

    pub fn put(&mut self, x: usize, y: usize, value: T) {
        for px in 0..3 {
            for py in 0..3 {
                self.grid[x + px][y + py].push(value.clone());
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Vec<T> {
        let mut res = Vec::new();
        for px in 0..3 {
            for py in 0..3 {
                for v in self.grid[x + px][y + py].iter() {
                    res.push(v.clone());
                }
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
        assert_eq!(grid.get(0, 0), vec!['a'].repeat(9));
    }
}
