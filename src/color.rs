#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

const U8_MAX_F: f64 = u8::MAX as f64;

impl Color {
    pub fn to_u32(&self) -> u32 {
        (self.r as u32 * 256 * 256) + (self.g as u32 * 256) + self.b as u32
    }

    #[allow(dead_code)]
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Converts HSV color to RGB color.
    ///
    /// # Arguments
    ///
    /// * `h` - Hue value (0-360)
    /// * `s` - Saturation value (0-1)
    /// * `v` - Value value (0-1)
    ///
    /// # Returns
    ///
    /// A `Color` struct representing the RGB color.
    pub fn hsv(h: f64, s: f64, v: f64) -> Color {
        let c = v * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

        let (r, g, b) = match h_prime.floor() as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            5 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let m = v - c;

        Color {
            r: ((r + m) * U8_MAX_F) as u8,
            g: ((g + m) * U8_MAX_F) as u8,
            b: ((b + m) * U8_MAX_F) as u8,
        }
    }

    pub fn mix(self, tr: f64, tg: f64, tb: f64) -> Color {
        Color {
            r: ((self.r as f64 * tr).min(U8_MAX_F)) as u8,
            g: ((self.g as f64 * tg).min(U8_MAX_F)) as u8,
            b: ((self.b as f64 * tb).min(U8_MAX_F)) as u8,
        }
    }
}
