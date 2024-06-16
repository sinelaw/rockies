use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct V2 {
    pub x: i32,
    pub y: i32,
}

impl V2 {
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
    pub fn cmul(&self, other: i32) -> V2 {
        V2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
    pub fn cdiv(&self, other: i32) -> V2 {
        V2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
    pub fn max(self, other: V2) -> V2 {
        V2 {
            x: cmp::max(self.x, other.x),
            y: cmp::max(self.y, other.y),
        }
    }
    pub fn min(self, other: V2) -> V2 {
        V2 {
            x: cmp::min(self.x, other.x),
            y: cmp::min(self.y, other.y),
        }
    }
}
