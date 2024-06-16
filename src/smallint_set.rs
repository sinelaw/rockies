pub(crate) struct IntPairSet {
    matrix: Vec<bool>,
    max_objs: usize,
}

impl IntPairSet {
    pub fn new(max_val: usize) -> IntPairSet {
        let mut matrix: Vec<bool> = Vec::new();
        matrix.resize(max_val * max_val, false);
        IntPairSet {
            matrix,
            max_objs: max_val,
        }
    }

    pub fn contains(&self, a: usize, b: usize) -> bool {
        self.matrix[a * self.max_objs + b]
    }

    pub fn put(&mut self, a: usize, b: usize) {
        self.matrix[a * self.max_objs + b] = true;
        self.matrix[b * self.max_objs + a] = true;
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
