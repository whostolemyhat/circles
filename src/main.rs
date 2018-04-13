// install cairo lib on machine
extern crate cairo;
extern crate rand;

use rand::distributions::{ IndependentSample, Range };
use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

pub mod utils;
pub mod colour;
pub mod shape;

use utils::Point;
use shape::{ Circle, Triangle, Shape };
use colour::Rgb;

const WIDTH: i32 = 2100;
const HEIGHT: i32 = 1400;

// don't know size of Shape
fn in_circle<T: ?Sized>(circle: &Box<T>, invert: bool) -> bool where T: Shape {
    let container_radius: f64 = 300.0;
    let point = circle.get_point();
    if utils::dist(point.x, point.y, WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0) > container_radius {
        if invert {
            return true;
        }
        return false;
    }

    if invert {
        return false;
    }
    true
}

fn collision<T: ?Sized>(circle: &Box<T>, shapes: &Vec<Box<T>>) -> bool where T: Shape {
    for s in shapes {
        if circle.collides(s.get_point(), s.get_size()) {
            return true;
        }
    }
    false
}

fn in_polygon(polygon_x_points: &Vec<f64>, polygon_y_points: &Vec<f64>, x: f64, y: f64, invert: bool) -> bool {
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

    if invert {
        return !c;
    }

    c
}

pub enum Particle {
    Circle,
    Triangle
}

pub enum Container {
    Circle,
    Star,
    Triangle,
    Rectangle
}

fn wobble_colour(colour: &Rgb) -> Rgb {
    let between = Range::new(0.0, 0.6);
    let darken_between = Range::new(0.4, 1.0);
    let mut rng = rand::thread_rng();
    let chance = between.ind_sample(&mut rng);
    let colour_wobble = match chance {
        x if x > 0.0 && x < 0.2 => colour.lighten(between.ind_sample(&mut rng)),
        x if x > 0.3 && x < 0.6 => colour.darken(darken_between.ind_sample(&mut rng)),
        _ => Rgb { r: colour.r, g: colour.g, b: colour.b }
    };

    colour_wobble
}

fn main() {
    let mut radius = 128.0;
    let radius_min: f64 = 8.0;
    let mut failed_tries = 0;
    let max_loop = 1_800_000;
    let max_tries = 128 * 1024;
    let container = Container::Circle;
    let invert = true;
    let particle = Particle::Circle;

    let surface = ImageSurface::create(Format::ARgb32, WIDTH, HEIGHT)
        .expect("Couldn't create surface");
    let context = Context::new(&surface);

    let mut rng = rand::thread_rng();


    let colour_range = Range::new(0.0, 1.0);

    let colour = Rgb {
        r: colour_range.ind_sample(&mut rng),
        g: colour_range.ind_sample(&mut rng),
        b: colour_range.ind_sample(&mut rng)
    };

    let mut circles = Vec::new();

    let (x_points, y_points) = match container {
        Container::Star => (vec![20.0, 95.0, 120.0, 145.0, 220.0, 170.0, 180.0, 120.0, 60.0, 70.0, 20.0], vec![85.0, 75.0, 10.0, 75.0, 85.0, 125.0, 190.0, 150.0, 190.0, 125.0, 85.0]),
        Container::Triangle => (vec![32.0, 200.0, 200.0], vec![32.0, 200.0, 32.0]),
        Container::Rectangle => (vec![150.0, 450.0, 450.0, 150.0], vec![300.0, 300.0, 100.0, 100.0]),
        Container::Circle => (vec![], vec![])
    };
    for _ in 0..max_loop {
        let between = Range::new(0.0, WIDTH as f64);
        let between_y = Range::new(0.0, HEIGHT as f64);

        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        let colour_wobble = wobble_colour(&colour);

        let shape: Box<Shape> = match particle {
            // need to wrap in Box to return different things which implement the same trait
            Particle::Circle => Box::new(Circle{ origin: Point { x, y }, size: radius, colour: colour_wobble }),
            Particle::Triangle => Box::new(Triangle{ origin: Point { x, y }, size: radius, colour: colour_wobble })
        };

        let mut flag = true;

        match container {
            Container::Circle => {
                if in_circle(&shape, invert) {
                    if !collision(&shape, &circles) {
                        circles.push(shape);
                    }
                } else {
                    failed_tries += 1;
                    if failed_tries as f64 > max_tries as f64 / radius {
                        radius /= 2.0;
                        failed_tries = 0;

                        if radius < radius_min {
                            flag = false;
                        }
                    }
                }
            }
            _ => {
                let point = shape.get_point();
                if in_polygon(&x_points, &y_points, point.x, point.y, invert) {
                    if !collision(&shape, &circles) {
                        circles.push(shape);
                    }
                } else {
                    failed_tries += 1;
                    if failed_tries as f64 > (32 * 1024) as f64 / radius {
                        radius /= 2.0;
                        failed_tries = 0;

                        if radius < radius_min {
                            flag = false;
                        }
                    }
                }
            }
        }

        if !flag {
            break;
        }
    }

    println!("{:?}", circles.len());
    for circle in circles {
        circle.draw(&context);
    }

    let mut file = File::create("circles.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}

// fn pack<T>(shape: T, radius: f64, circles: Vec<T>, flag: bool, x_points: Vec<f64>, y_points: Vec<f64>, invert: bool) where T: Shape {
//     let radius_min: f64 = 2.0;
//     let mut failed_tries = 0;

//     let mut circles = Vec::new();

//     // if in_circle(&shape, invert) {
//     //     if !collision(&shape, &circles) {
//     //         circles.push(shape);
//     //     }
//     // } else {
//     let point = shape.get_point();
//     if in_polygon(x_points, y_points, point.x, point.y, invert) {
//         if !collision(&shape, &circles) {
//             circles.push(shape);
//         }
//     } else {
//         failed_tries += 1;
//         if failed_tries as f64 > (32 * 1024) as f64 / radius {
//             radius /= 2.0;
//             failed_tries = 0;

//             if radius < radius_min {
//                 flag = false;
//             }
//         }
//     }
// }