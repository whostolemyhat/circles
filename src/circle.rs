// extern crate cairo;
// extern crate rand;

// use cairo::{ Context };
// // use rand::distributions::{ IndependentSample, Range };
// use utils::{ Point, dist };
// use colour::Rgb;
// use Collides;
// use HasSize;
// use HasPoint;

// const MARGIN: f64 = 0.5;

// pub struct Circle {
//     colour: Rgb,
//     pub x: f64,
//     pub y: f64,
//     radius: f64
// }

// impl Circle {
//     pub fn new(x: f64, y: f64, radius: f64, colour: Rgb) -> Self {
//         // let between = Range::new(0.0, 0.6);
//         // let darken_between = Range::new(0.4, 1.0);
//         // let mut rng = rand::thread_rng();
//         // let chance = between.ind_sample(&mut rng);

//         // let colour = match chance {
//         //     x if x > 0.0 && x < 0.2 => base_colour.lighten(between.ind_sample(&mut rng)),
//         //     x if x > 0.3 && x < 0.6 => base_colour.darken(darken_between.ind_sample(&mut rng)),
//         //     _ => Rgb { r: base_colour.r, g: base_colour.g, b: base_colour.b }
//         // };

//         Circle { x, y, radius, colour }
//     }

//     pub fn draw(&self, context: &Context) {
//         context.set_source_rgb(self.colour.r, self.colour.g, self.colour.b);
//         context.arc(self.x, self.y, self.radius, 0.0, 360.0);
//         context.fill();
//     }
// }

// impl HasSize for Circle {
//     fn size(&self) -> f64 {
//         self.radius
//     }
// }

// impl HasPoint for Circle {
//     fn get_point(&self) -> Point {
//         Point { x: self.x, y: self. y }
//     }
// }

// impl Collides<Circle> for Circle {
//     fn collides(&self, other: &Circle) -> bool {
//         dist(self.x, self.y, other.x, other.y) < (self.radius + other.size() + MARGIN)
//     }
// }