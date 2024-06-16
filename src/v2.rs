use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V2 {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl V2 {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn min(&self, other: V2) -> V2 {
        V2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
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
            x: (self.x as f64 * other) as f64,
            y: (self.y as f64 * other) as f64,
        }
    }
    pub fn cdiv(&self, other: f64) -> V2 {
        V2 {
            x: (self.x as f64 / other) as f64,
            y: (self.y as f64 / other) as f64,
        }
    }

    pub fn dot(&self, other: V2) -> f64 {
        self.x * other.x + self.y * other.y
    }
}
