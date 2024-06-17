pub(crate) struct IntPairSet {
    matrix: Vec<(u8, bool)>,
    version: u8,
    max_objs: usize,
}

impl IntPairSet {
    pub fn new(max_val: usize) -> IntPairSet {
        let mut matrix: Vec<(u8, bool)> = Vec::new();
        matrix.resize(max_val * max_val, (0, false));
        IntPairSet {
            matrix,
            version: 0,
            max_objs: max_val,
        }
    }

    pub fn contains(&self, a: usize, b: usize) -> bool {
        let (version, value) = self.matrix[a * self.max_objs + b];
        (version == self.version) && value
    }

    pub fn put(&mut self, a: usize, b: usize) {
        self.matrix[a * self.max_objs + b] = (self.version, true);
        self.matrix[b * self.max_objs + a] = (self.version, true);
    }

    pub fn clear(&mut self) {
        self.version += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_pair_set_contains_empty() {
        let set = IntPairSet::new(5);
        assert!(!set.contains(2, 3));
    }

    #[test]
    fn test_int_pair_set_contains_after_put() {
        let mut set = IntPairSet::new(5);
        set.put(2, 3);
        assert!(set.contains(2, 3));
        assert!(set.contains(3, 2));
    }

    #[test]
    fn test_int_pair_set_put_idempotent() {
        let mut set = IntPairSet::new(5);
        set.put(2, 3);
        set.put(2, 3);
        assert!(set.contains(2, 3));
        assert!(set.contains(3, 2));
    }
}
