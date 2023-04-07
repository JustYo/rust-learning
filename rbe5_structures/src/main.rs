#![allow(dead_code)]
use std::fmt::{self, Display, Formatter};

// A unit struct
struct Unit;

//A tuple struct
struct Pair(i32, f32);

//A struct with fields
struct Point {
    x: f32,
    y: f32,
}

//Structs can be reused as fields of another struct

/// Rectangle structure
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn pow2(num: f32) -> f32 {
    let pow = num * num;
    pow
}

fn react_area(rectangle: Rectangle) -> f32 {
    // Formula = d=sqrt(sqrt(x2-x1) + sqrt(y2-y1))
    let result = (pow2(rectangle.top_left.y - rectangle.bottom_right.y)
        + pow2(rectangle.top_left.x - rectangle.bottom_right.x))
    .sqrt();
    result
}

// Formatting display for a rectangle
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Rectangle Description: ")?;
        writeln!(
            f,
            "Top left coordonates: ({} {})",
            self.top_left.x, self.top_left.y
        )?;
        write!(
            f,
            "Right bottom coordonates: ({} {})",
            self.bottom_right.x, self.bottom_right.y
        )
    }
}

///Square structure
fn square(rectangle_corner: Point, num: f32) -> Rectangle {
    let new_point = Point {
        x: rectangle_corner.x + num,
        y: rectangle_corner.y - num,
    };
    let rectangle = Rectangle {
        top_left: rectangle_corner,
        bottom_right: new_point,
    };
    rectangle
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    println!("{}", _rectangle);
    println!("Area: {}", react_area(_rectangle));
    println!("Carré associé: {}", square(point, 3.0))
}
