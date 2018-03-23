extern crate cairo;
// extern crate nalgebra;

use colour::Rgb;
use cairo::{ Context };
use Collides;
use HasSize;
use HasPoint;
// use self::nalgebra::{ Matrix2x3, Matrix1x3 };
use utils::Point;

#[derive(Debug)]
pub struct Triangle {
    pub x: f64,
    pub y: f64,
    pub colour: Rgb,
    pub size: f64
}

impl Triangle {
    pub fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.r, self.colour.g, self.colour.b);
        context.new_path();
        context.move_to(self.x, self.y);
        context.line_to(self.x - self.size, self.y);
        context.line_to(self.x, self.y - self.size);
        context.close_path();
        context.fill();
    }

    pub fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        points.push(Point { x: self.x, y: self.y });
        points.push(Point { x: self.x - self.size, y: self.y });
        points.push(Point { x: self.x, y: self.y - self.size });

        points
    }
}

impl HasSize for Triangle {
    fn size(&self) -> f64 {
        self.size
    }
}

impl HasPoint for Triangle {
    fn get_point(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

fn dot(u: &Vec<f64>, v: &Vec<f64>) -> f64 {
    u[0] * v[0] + u[1] * v[1]
}

fn make_product(from: &Point, to: &Point) -> Vec<f64> {
    vec![to.x - from.x, to.y - from.y]
}

// point: point to check
// a, b, c: points of triangle
fn point_in_triangle(point: &Point, a: &Point, b: &Point, c: &Point) -> bool {
    let v0 = make_product(&a, &c);
    let v1 = make_product(a, b);
    let v2 = make_product(a, &point);

    let dot00 = dot(&v0, &v0);
    let dot01 = dot(&v0, &v1);
    let dot02 = dot(&v0, &v2);
    let dot11 = dot(&v1, &v1);
    let dot12 = dot(&v1, &v2);

    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    // is point in triangle?
    (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
}

// https://stackoverflow.com/questions/2464902/determine-if-a-point-is-inside-a-triangle-formed-by-3-points-with-given-latitude
impl Collides<Triangle> for Triangle {
    fn collides(&self, other: &Triangle) -> bool {
        let points = self.points();
        let other_points = other.points();

        for point in &points {
            if point_in_triangle(&point, &other_points[0], &other_points[1], &other_points[2]) {
                return true;
            }
        }

        for point in other_points {
            if point_in_triangle(&point, &points[0], &points[1], &points[2]) {
                return true;
            }
        }

        false
    }

}