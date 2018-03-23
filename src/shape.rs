use colour::Rgb;
use utils::Point;
use cairo::Context;
use utils::dist;

const MARGIN: f64 = 0.5;

pub trait Shape {
    fn new(x: f64, y: f64, size: f64, colour: Rgb) -> Self;
    fn draw(&self, context: &Context);
    fn collides(&self, other: &Self) -> bool;
    fn get_point(&self) -> Point;
}

pub struct Circle {
    colour: Rgb,
    origin: Point,
    size: f64
}
impl Shape for Circle {
    fn new(x: f64, y: f64, size: f64, colour: Rgb) -> Self {
        Self { origin: Point { x, y }, size, colour }
    }

    fn get_point(&self) -> Point {
        self.origin.clone()
    }

    fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.r, self.colour.g, self.colour.b);
        context.arc(self.origin.x, self.origin.y, self.size, 0.0, 360.0);
        context.fill();
    }

    fn collides(&self, other: &Circle) -> bool {
        dist(self.origin.x, self.origin.y, other.origin.x, other.origin.y) < (self.size + other.size + MARGIN)
    }
}

pub struct Triangle {
    colour: Rgb,
    origin: Point,
    size: f64
}
impl Shape for Triangle {
    fn new(x: f64, y: f64, size: f64, colour: Rgb) -> Self {
        Triangle { origin: Point { x, y }, size, colour }
    }

    fn get_point(&self) -> Point {
        self.origin.clone()
    }

    fn draw(&self, context: &Context) {
        context.set_source_rgb(self.colour.r, self.colour.g, self.colour.b);
        context.new_path();
        context.move_to(self.origin.x, self.origin.y);
        context.line_to(self.origin.x - self.size, self.origin.y);
        context.line_to(self.origin.x, self.origin.y - self.size);
        context.close_path();
        context.fill();
    }

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

impl Triangle {
    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        points.push(Point { x: self.origin.x, y: self.origin.y });
        points.push(Point { x: self.origin.x - self.size, y: self.origin.y });
        points.push(Point { x: self.origin.x, y: self.origin.y - self.size });

        points
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

