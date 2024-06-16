use std::cmp;

use web_sys::js_sys::Math;

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
    pub fn cmul(&self, other: f64) -> V2 {
        V2 {
            x: Math::round(self.x as f64 * other) as i32,
            y: Math::round(self.y as f64 * other) as i32,
        }
    }
    pub fn cdiv(&self, other: f64) -> V2 {
        V2 {
            x: Math::round(self.x as f64 / other) as i32,
            y: Math::round(self.y as f64 / other) as i32,
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
    pub fn dot(&self, other: V2) -> i32 {
        self.x * other.x + self.y * other.y
    }
}
