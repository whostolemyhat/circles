extern crate cairo;
extern crate rand;

use cairo::{ Context };
use rand::distributions::{ IndependentSample, Range };
use utils;

const MARGIN: f64 = 0.5;

pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

pub struct Circle {
    colour: Rgb,
    pub x: f64,
    pub y: f64,
    radius: f64
}

fn darken(colour: &Rgb, amount: f64) -> Rgb {
    Rgb {
        r: colour.r * amount,
        g: colour.g * amount,
        b: colour.b * amount
    }
}

fn lighten(colour: &Rgb, amount: f64) -> Rgb {
    Rgb {
        r: colour.r + (1.0 - colour.r) * amount,
        g: colour.g + (1.0 - colour.g) * amount,
        b: colour.g + (1.0 - colour.g) * amount
    }
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64, base_colour: &Rgb) -> Self {
        let between = Range::new(0.0, 0.6);
        let darken_between = Range::new(0.4, 1.0);
        let mut rng = rand::thread_rng();
        let chance = between.ind_sample(&mut rng);

        let colour = match chance {
            x if x > 0.0 && x < 0.2 => lighten(&base_colour, between.ind_sample(&mut rng)),
            x if x > 0.3 && x < 0.6 => darken(&base_colour, darken_between.ind_sample(&mut rng)),
            _ => Rgb { r: base_colour.r, g: base_colour.g, b: base_colour.b }
        };

        Circle { x, y, radius, colour }
    }

    pub fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.r, self.colour.g, self.colour.b);
        context.arc(self.x, self.y, self.radius, 0.0, 360.0);
        context.fill();
    }

    pub fn collides(&self, other: &Circle) -> bool {
        utils::dist(self.x, self.y, other.x, other.y) < (self.radius + other.radius + MARGIN)
    }
}