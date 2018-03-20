extern crate cairo;
extern crate rand;

use cairo::{ Context };
use rand::distributions::{ IndependentSample, Range };
use utils;

const MARGIN: f64 = 2.0;

pub struct Circle {
    colour: (f64, f64, f64),
    pub x: f64,
    pub y: f64,
    radius: f64
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        let between = Range::new(0.0, 1.0);
        let mut rng = rand::thread_rng();

        let colour = (between.ind_sample(&mut rng), between.ind_sample(&mut rng), between.ind_sample(&mut rng));

        Circle::new_with_colour(x, y, radius, colour)
    }

    fn new_with_colour(x: f64, y: f64, radius: f64, colour: (f64, f64, f64)) -> Self {
        Circle{
            x,
            y,
            radius,
            colour
        }
    }

    pub fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.0, self.colour.1, self.colour.2);
        context.arc(self.x, self.y, self.radius, 0.0, 360.0);
        context.fill();
    }

    pub fn collides(&self, other: &Circle) -> bool {
        utils::dist(self.x, self.y, other.x, other.y) < (self.radius + other.radius + MARGIN)
    }
}