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

fn collision(circle: &Circle, circles: &Vec<Circle>) -> bool {
    for c in circles {
        if circle.collides(c) {
            return true;
        }
    }
    false
}

fn in_polygon(polygon_x_points: Vec<f64>, polygon_y_points: Vec<f64>, x: f64, y: f64) -> bool {
    let mut c = false;
    let mut j = polygon_x_points.len() - 1;

    for i in 0..polygon_x_points.len() {
        let delta_x = polygon_x_points[j] - polygon_x_points[i];
        let y_spread = y - polygon_y_points[i];
        let delta_y = polygon_y_points[j] - polygon_y_points[i];
        if ((polygon_y_points[i] > y) != (polygon_y_points[j] > y)) && (x < (((delta_x * y_spread) / delta_y) + polygon_x_points[i])) {
            c = !c;
        }

        j = i;
    }

    c
}

fn main() {
    let mut radius = 16.0;

    let surface = ImageSurface::create(Format::ARgb32, WIDTH, HEIGHT)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);
    let mut circles: Vec<Circle> = vec![];
    let mut rng = rand::thread_rng();

    let mut failed_tries = 0;

    for _ in 0..2_000_000 {
        let between = Range::new(0.0, WIDTH as f64);
        let between_y = Range::new(0.0, HEIGHT as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        let x_points = vec![32.0, 200.0, 200.0, 32.0];
        let y_points = vec![200.0, 200.0, 32.0, 32.0];

        let circle = Circle::new(x, y, radius);

        // if is_valid(&circle, &circles) {
        //     circles.push(circle);
        // } else {
        if in_polygon(x_points, y_points, circle.x, circle.y) {
            if !(collision(&circle, &circles)) {
                circles.push(circle);
            }
        } else {
            failed_tries += 1;
            if failed_tries as f64 > (24 * 1024) as f64 / radius {

                println!("{:?} {:?}", failed_tries, (24 * 1024) as f64 / radius);

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