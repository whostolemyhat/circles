// install cairo lib on machine
extern crate cairo;

use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

struct Circle {
    colour: (f64, f64, f64),
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64, colour: (f64, f64, f64)) -> Self {
        Circle{
            x,
            y,
            radius,
            colour
        }
    }

    fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.0, self.colour.1, self.colour.2);
        context.arc(self.x, self.y, self.radius, 0.0, 360.0);
        context.fill();
    }
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    let circle = Circle::new(50.0, 75.0, 15.0, (1.0, 0.3, 0.4));
    circle.draw(&context);

    let mut file = File::create("circles.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}