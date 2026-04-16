#![allow(dead_code)]
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

struct Point<T> {
    pub x: T,
    pub y: T,
}

struct Line {
    pub a: Point<f64>,
    pub b: Point<f64>,
}

// ** START EDITS HERE **

// create a trait OriginDistance that declares a single method: distance_from_origin(&self) -> f64;
// then implement that trait for both Point<f64> and Line
// distance from 0,0 = sqrt(x^2 + y^2)
// for line, find the shortest distance from origin of the two points (a and b)
trait OriginDistance {
    fn distance_from_origin(&self) -> f64;
}

impl OriginDistance for Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl OriginDistance for Line {
    fn distance_from_origin(&self) -> f64 {
        self.a
            .distance_from_origin()
            .min(self.b.distance_from_origin())
    }
}

// a little tricky but you got it
// implement a method `sort_and_format` for any Point<T> where T is bounded by the std::fmt::Display and std::cmp::PartialOrd traits
// PartialOrd allows us to use <, >, >=, and <= operators. `sort_and_format` should take &self and &other: &Point<T> as parameters
// and return a string of the largest ({x}, {y}) combination followed by the smallest ({x}, {y}) combination
// i.e. if self.x = 7, self.y = 4, other.x = 5, other.y = 7 then return "(7, 7) (5, 4)"
// in the case where we can't compare values (i.e. NaN) return an empty string
// hint: use .partial_cmp() to compare self and other x and y values, and the format! macro to return a formatted string.
// standard library documentation might be useful for this one!
impl<T> Point<T>
where
    T: fmt::Display + PartialOrd,
{
    pub fn sort_and_format(&self, other: &Point<T>) -> String {
        let xs = match &self.x.partial_cmp(&other.x) {
            Some(Ordering::Less) => Some((&other.x, &self.x)),
            None => None,
            // Greater or equal to => self.x, other.x
            _ => Some((&self.x, &other.x)),
        };
        let ys = match &self.y.partial_cmp(&other.y) {
            Some(Ordering::Less) => Some((&other.y, &self.y)),
            None => None,
            // Greater or equal to => self.y, other.y
            _ => Some((&self.y, &other.y)),
        };

        // We can do a match on tuples (of tuples) in Rust! We try to avoid using .unwrap() when possible
        match (xs, ys) {
            (Some((x_1, x_2)), Some((y_1, y_2))) => format!("({x_1}, {y_1}) ({x_2}, {y_2})"),
            _ => "".to_string(),
        }
    }
}

// ** END EDITS HERE **

// Try not to look at tests until you finished your implementations :)
#[cfg(test)]
pub mod tests {
    use std::f64;

    use crate::vid_8::OriginDistance;

    #[test]
    fn distance_from_origin_point() {
        let coords: [(f64, f64); 5] =
            [(4.0, 3.2), (-7.5, 0.0), (3.3, -8.0), (0.0, 5.0), (0.0, 0.0)];
        for (x, y) in coords {
            let p = super::Point { x: x, y: y };
            let dist = (x.powi(2) + y.powi(2)).sqrt();
            assert_eq!(&p.distance_from_origin(), &dist);
        }
    }

    #[test]
    fn distance_from_origin_line() {
        let coords: [[(f64, f64); 2]; 5] = [
            [(0.0, 0.0), (1.0, 1.0)],
            [(-3.5, 4.0), (7.0, 1.24)],
            [(4.0, 10.053), (-1.1, 4.04)],
            [(5.0, 0.0), (5.0, -1.0)],
            [(0.0, 0.0), (0.0, 0.0)],
        ];
        for c in coords {
            let line = super::Line {
                a: super::Point {
                    x: c[0].0,
                    y: c[0].1,
                },
                b: super::Point {
                    x: c[1].0,
                    y: c[1].1,
                },
            };
            let dist = line
                .a
                .distance_from_origin()
                .min(line.b.distance_from_origin());

            assert_eq!(line.distance_from_origin(), dist);
        }
    }

    #[test]
    fn sort_and_format() {
        // i32: (x_b, y_a) (x_a, y_b)
        let a = super::Point { x: 505, y: 202 };
        let b = super::Point { x: 600, y: 200 };
        let sorted_output = a.sort_and_format(&b);
        assert_eq!(&sorted_output, &"(600, 202) (505, 200)".to_string());

        // f64: (x_b, y_b) (x_a, y_a)
        let a = super::Point { x: 4.3, y: 0.4 };
        let b = super::Point { x: 6.0, y: 1.2 };
        let sorted_output = a.sort_and_format(&b);
        assert_eq!(&sorted_output, &"(6, 1.2) (4.3, 0.4)".to_string());

        // f64: NaN
        let a = super::Point { x: 4.3, y: 0.4 };
        let b = super::Point {
            x: 6.0,
            y: f64::NAN,
        };
        let sorted_output = a.sort_and_format(&b);
        assert_eq!(&sorted_output, &"".to_string());

        // char: (x_a, y_a) (x_b, y_b)
        let a = super::Point { x: 'x', y: 'y' };
        let b = super::Point { x: 'a', y: 'b' };
        let sorted_output = a.sort_and_format(&b);
        assert_eq!(&sorted_output, &"(x, y) (a, b)".to_string());

        // bool: (x_a_or_b, y_b) (x_a_or_b, y_a)
        let a = super::Point { x: true, y: false };
        let b = super::Point { x: true, y: true };
        let sorted_output = a.sort_and_format(&b);
        assert_eq!(&sorted_output, &"(true, true) (true, false)".to_string());
    }
}
