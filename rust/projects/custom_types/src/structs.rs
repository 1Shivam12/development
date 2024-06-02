#![allow(dead_code)]

use std::fmt;
// An attribute to hide warnings for unused code

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with 2 fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be re-used as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This Person's name is {}\nThis Person's Age is {}", self.name, self.age)
    }
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle {top_left: Point { x : top_left_x, y : top_left_y }, bottom_right: Point {x: bottom_right_x, y: bottom_right_y}} = rectangle;

    ((bottom_right_x - top_left_x)*(bottom_right_y-top_left_y)).abs()
}

fn square(point: Point, dim: f32) -> Rectangle {
    let Point { x : top_left_x, y : top_left_y } = point;
    let bottom_right_point: Point = Point { x : top_left_x + dim, y : top_left_y - dim};

    Rectangle {top_left : point, bottom_right : bottom_right_point}
}

pub fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);


    let point: Point = Point { x : 1.1, y :1.2};
    let another_point: Point = Point { x : 5.2, y : 0.2};

    println!("Access point coordinates {} {}", point.x, point.y); 

    let bottom_right: Point = Point { x:5.2, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure point
    let Point {x: left_edge, y: top_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let area = rect_area(_rectangle);

    println!("The area of the rectangle is {}", area);

    let square = square(point, 5.0);
    let square_area = rect_area(square);
    println!("The area of the square is {}", square_area);

}
