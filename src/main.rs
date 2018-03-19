// install cairo lib on machine
extern crate cairo;
extern crate rand;

use rand::distributions::{ IndependentSample, Range };
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

const MARGIN: f64 = 5.0;

#[derive(Debug)]
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

    fn collides(&self, other: &Circle) -> bool {
        dist(self.x, self.y, other.x, other.y) < (self.radius + other.radius + MARGIN)
    }
}

fn is_valid(circle: &Circle, circles: &Vec<Circle>) -> bool {
    // width / 2, height / 2
    if dist(circle.x, circle.y, 300.0, 200.0) > 200.0 {
        return false;
    }

    for c in circles {
        if circle.collides(c) {
            return false;
        }
    }
    true
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn main() {
    let width = 600;
    let height = 400;
    let mut radius = 16.0;
    let RADIUS_MIN = 2.0;

    let surface = ImageSurface::create(Format::ARgb32, width, height)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);
    let mut circles: Vec<Circle> = vec![];
    let mut rng = rand::thread_rng();

    let mut failed_tries = 0;

    for _ in 0..20000 {
        // if failed_tries < 32 * 1024 / radius {
        let between = Range::new(0.0, width as f64);
        let between_y = Range::new(0.0, height as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        let circle = Circle::new(x, y, radius);
        if is_valid(&circle, &circles) {
            circles.push(circle);
            // }
        } else {
            failed_tries += 1;
            if failed_tries as f64 > (32 * 1024) as f64 / radius {
                radius /= 2.0;
                failed_tries = 0;

                if radius < RADIUS_MIN {
                    break;
                }
            }
        }
    }

    println!("{:?}", circles.len());
    for circle in circles {
        circle.draw(&context);
    }

    let mut file = File::create("circles.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}