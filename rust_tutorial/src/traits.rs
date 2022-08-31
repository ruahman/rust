trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}
struct Rectangle {
    length: f32,
    width: f32,
}
struct Circle {
    length: f32,
    width: f32,
}
const PI: f32 = 3.14;

impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
        return Rectangle { length, width };
    }
    fn area(&self) -> f32 {
        return self.length * self.width;
    }
}
impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
        return Circle { length, width };
    }
    fn area(&self) -> f32 {
        return (self.length / 2.0).powf(2.0) * PI;
    }
}

pub fn demo() {
    let rec: Rectangle = Shape::new(44.5, 66.7);
    let circle: Circle = Shape::new(22.2, 77.7);
    println!("{}{}", rec.length, rec.width);
    println!("{}{}", circle.length, circle.width);
    println!("{}", rec.area());
    println!("{}", circle.area());
}
