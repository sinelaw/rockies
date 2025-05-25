#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Hash, Copy, Debug, PartialEq, Eq)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
}

impl V2i {
    pub fn new(x: i32, y: i32) -> V2i {
        V2i { x, y }
    }

    pub fn to_v2(&self) -> V2 {
        V2 {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn plus(&self, offset: V2i) -> V2i {
        V2i {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn minus(&self, offset: V2i) -> V2i {
        V2i {
            x: self.x - offset.x,
            y: self.y - offset.y,
        }
    }
}

fn round(x: f64) -> i32 {
    x.round() as i32
}

impl V2 {
    pub fn new(x: f64, y: f64) -> V2 {
        V2 { x, y }
    }

    pub fn zero() -> V2 {
        V2 { x: 0.0, y: 0.0 }
    }

    pub fn round(&self) -> V2i {
        V2i {
            x: round(self.x),
            y: round(self.y),
        }
    }
    pub fn magnitude(&self) -> f64 {
        self.magnitude_sqr().sqrt()
    }

    pub fn magnitude_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn min(&self, other: V2) -> V2 {
        V2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(&self, other: V2) -> V2 {
        V2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn plus(&self, other: V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn minus(&self, other: V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn cmul(&self, other: f64) -> V2 {
        V2 {
            x: (self.x * other),
            y: (self.y * other),
        }
    }
    pub fn cdiv(&self, other: f64) -> V2 {
        V2 {
            x: (self.x / other),
            y: (self.y / other),
        }
    }

    pub fn dot(&self, other: V2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v2_magnitude() {
        let v = V2 { x: 3.0, y: 4.0 };
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_v2_min() {
        let v1 = V2 { x: 3.0, y: 4.0 };
        let v2 = V2 { x: 1.0, y: 2.0 };
        assert_eq!(v1.min(v2), V2 { x: 1.0, y: 2.0 });
    }

    #[test]
    fn test_v2_plus() {
        let v1 = V2 { x: 3.0, y: 4.0 };
        let v2 = V2 { x: 1.0, y: 2.0 };
        assert_eq!(v1.plus(v2), V2 { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_v2_minus() {
        let v1 = V2 { x: 3.0, y: 4.0 };
        let v2 = V2 { x: 1.0, y: 2.0 };
        assert_eq!(v1.minus(v2), V2 { x: 2.0, y: 2.0 });
    }

    #[test]
    fn test_v2_cmul() {
        let v = V2 { x: 3.0, y: 4.0 };
        assert_eq!(v.cmul(2.0), V2 { x: 6.0, y: 8.0 });
    }

    #[test]
    fn test_v2_cdiv() {
        let v = V2 { x: 6.0, y: 8.0 };
        assert_eq!(v.cdiv(2.0), V2 { x: 3.0, y: 4.0 });
    }

    #[test]
    fn test_v2_dot() {
        let v1 = V2 { x: 3.0, y: 4.0 };
        let v2 = V2 { x: 1.0, y: 2.0 };
        assert_eq!(v1.dot(v2), 11.0);
    }

    #[test]
    fn test_v2_round() {
        let v = V2 { x: 1.2, y: 2.8 };
        let v2i = v.round();
        assert_eq!(v2i.x, 1);
        assert_eq!(v2i.y, 3);
    }

    #[test]
    fn test_v2_round_negative() {
        let v = V2 { x: -1.0, y: -1.0 };
        let v2i = v.round();
        assert_eq!(v2i.x, -1);
        assert_eq!(v2i.y, -1);
    }
}
