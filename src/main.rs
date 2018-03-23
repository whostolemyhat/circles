// install cairo lib on machine
extern crate cairo;
extern crate rand;

use rand::distributions::{ IndependentSample, Range };
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

pub mod circle;
pub mod triangle;
pub mod utils;
pub mod colour;

use circle::Circle;
use triangle::Triangle;
use colour::Rgb;

const RADIUS_MIN: f64 = 2.0;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

trait Collides<T: HasSize> {
    fn collides(&self, other: &T) -> bool;
}

trait HasSize {
    fn size(&self) -> f64;
}

// fn in_circle(circle: &Circle, circles: &Vec<Circle>) -> bool {
    // let CONTAINER_RADIUS: f64 = 180.0;
//     if utils::dist(circle.x, circle.y, WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0) > CONTAINER_RADIUS {
//         return false;
//     }

//     if collision(&circle, &circles) {
//         return false;
//     }

//     true
// }

fn collision<T: Collides<T>>(circle: &T, shapes: &Vec<T>) -> bool where T: HasSize{
    for s in shapes {
        if circle.collides(s) {
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
    let mut radius = 32.0;

    let surface = ImageSurface::create(Format::ARgb32, WIDTH, HEIGHT)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);
    // let mut circles: Vec<Circle> = vec![];
    let mut circles: Vec<Triangle> = vec![];
    let mut rng = rand::thread_rng();

    let mut failed_tries = 0;

    let colour_range = Range::new(0.0, 1.0);

    let colour = Rgb {
        r: colour_range.ind_sample(&mut rng),
        g: colour_range.ind_sample(&mut rng),
        b: colour_range.ind_sample(&mut rng)
    };

    // let colour = Rgb {
    //     r: 0.8,
    //     g: 0.2,
    //     b: 0.1
    // };


    for _ in 0..2_000_000 {
    // for i in 0..40 {
        let between = Range::new(0.0, WIDTH as f64);
        let between_y = Range::new(0.0, HEIGHT as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        // containers - for circle use in_circle not in_polygon
        // rect
        let x_points = vec![150.0, 450.0, 450.0, 150.0];
        let y_points = vec![300.0, 300.0, 100.0, 100.0];

        // triangle
        // let x_points = vec![32.0, 100.0, 100.0];
        // let y_points = vec![32.0, 100.0, 32.0];

        // star
        // let x_points = vec![20.0, 95.0, 120.0, 145.0, 220.0, 170.0, 180.0, 120.0, 60.0, 70.0, 20.0];
        // let y_points = vec![85.0, 75.0, 10.0, 75.0, 85.0, 125.0, 190.0, 150.0, 190.0, 125.0, 85.0];

        let between = Range::new(0.0, 0.6);
        let darken_between = Range::new(0.4, 1.0);
        let mut rng = rand::thread_rng();
        let chance = between.ind_sample(&mut rng);
        let colour_wobble = match chance {
            x if x > 0.0 && x < 0.2 => colour.lighten(between.ind_sample(&mut rng)),
            x if x > 0.3 && x < 0.6 => colour.darken(darken_between.ind_sample(&mut rng)),
            _ => Rgb { r: colour.r, g: colour.g, b: colour.b }
        };

        // let circle = Circle::new(x, y, radius, colour_wobble);
        let circle = Triangle { x, y, size: radius, colour: colour_wobble };

        // if in_circle(&circle, &circles) {
        //     circles.push(circle);
        // } else {
        if in_polygon(x_points, y_points, circle.x, circle.y) {
            if !(collision(&circle, &circles)) {
                circles.push(circle);
            }
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
        // println!("{:?}", circle.points());
        circle.draw(&context);
    }

    let mut file = File::create("circles.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}