#![allow(dead_code)]
#![allow(unused_variables)]

// there are three types of structres("structs") that can be created using
// the struct keyword
// - tuple struct
// - classic c struct
// - Unit struct, which are field-less

// tuple struct
struct Pair(i32, f32);

// classic c struct
#[derive(Debug)]
struct PersonClasic {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// a struct with two fields
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
struct RectangleReused {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

struct Line {
    start: Point,
    end: Point,
}

fn stuctures() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p, end: p2 };
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct TupColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // this is called an associated function,
    // since it doesn't have a reference to an instance of the struct
    fn new(first: &str, last: &str) -> Self {
        Self {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }
}

struct Customer {
    name: String,
    address: String,
    balance: f32,
}

struct Rectangle<T, U> {
    width: T,
    height: U,
}

pub fn structs() {
    stuctures();

    let mut bob = Customer {
        name: String::from("bob"),
        address: String::from("123 main"),
        balance: 0.0,
    };
    bob.address = String::from("123 main st");

    let rec = Rectangle {
        width: 10,
        height: 20.5,
    };

    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("{}{}{}", c.red, c.green, c.blue);

    let x = TupColor(1, 2, 3);

    println!("{}{}{}", x.0, x.1, x.2);

    let mut p = Person::new("diego", "vila");

    println!("Person {}{}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("bennet");

    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = PersonClasic { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = RectangleReused {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        structs()
    }
}
