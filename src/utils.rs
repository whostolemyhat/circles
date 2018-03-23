#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
