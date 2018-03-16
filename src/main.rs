// install cairo lib on machine
extern crate cairo;
extern crate rand;

use rand::distributions::{ IndependentSample, Range };
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

struct Circle {
    colour: (f64, f64, f64),
    x: f64,
    y: f64,
    radius: f64
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Self {
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

    fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.0, self.colour.1, self.colour.2);
        context.arc(self.x, self.y, self.radius, 0.0, 360.0);
        context.fill();
    }
}

fn main() {
    let width = 600;
    let height = 400;

    let surface = ImageSurface::create(Format::ARgb32, width, height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);
    let mut circles: Vec<Circle> = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let between = Range::new(0.0, width as f64);
        let between_y = Range::new(0.0, height as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        let circle = Circle::new(x, y, 15.0);
        circles.push(circle);
    }

    for circle in circles {
        circle.draw(&context);
    }

    let mut file = File::create("circles.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}