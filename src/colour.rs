#[derive(Debug)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Rgb { r, g, b }
    }

    pub fn darken(&self, amount: f64) -> Self {
        Rgb {
            r: self.r * amount,
            g: self.g * amount,
            b: self.b * amount
        }
    }

    pub fn lighten(&self, amount: f64) -> Self {
        Rgb {
            r: self.r + (1.0 - self.r) * amount,
            g: self.g + (1.0 - self.g) * amount,
            b: self.g + (1.0 - self.g) * amount
        }
    }
}