// install cairo lib on machine
extern crate cairo;
extern crate rand;

use rand::distributions::{ IndependentSample, Range };
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

pub mod circle;
pub mod utils;
use circle::Circle;

const RADIUS_MIN: f64 = 2.0;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;
const CONTAINER_RADIUS: f64 = 200.0;

fn is_valid(circle: &Circle, circles: &Vec<Circle>) -> bool {
    if utils::dist(circle.x, circle.y, WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0) > CONTAINER_RADIUS {
        return false;
    }

    for c in circles {
        if circle.collides(c) {
            return false;
        }
    }
    true
}

fn main() {
    let mut radius = 16.0;

    let surface = ImageSurface::create(Format::ARgb32, WIDTH, HEIGHT)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);
    let mut circles: Vec<Circle> = vec![];
    let mut rng = rand::thread_rng();

    let mut failed_tries = 0;

    for _ in 0..20000 {
        let between = Range::new(0.0, WIDTH as f64);
        let between_y = Range::new(0.0, HEIGHT as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        let circle = Circle::new(x, y, radius);
        if is_valid(&circle, &circles) {
            circles.push(circle);
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