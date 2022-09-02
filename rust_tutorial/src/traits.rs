use std::fmt::Debug;
use std::ops::Add;

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}
struct Rectangle {
    length: f32,
    width: f32,
}

#[derive(Debug)]
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

fn print_info(shape: impl Shape + Debug) {
    println!("the area is {}", shape.area());
}

#[allow(dead_code)]
fn print_info2<T: Shape + Debug>(shape: T) {
    println!("{}", shape.area())
}

#[allow(dead_code)]
fn print_info3<T>(shape: T)
where
    T: Shape + Debug,
{
    println!("{}", shape.area())
}

#[derive(Debug)]
struct Person {
    name: String,
}

// every object it excepts as a parameter,
// must have Into<String> implemented
impl Person {
    fn new<S>(name: S) -> Person
    where
        S: Into<String>,
    {
        Person { name: name.into() }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop person");
    }
}

impl Add for Person {
    type Output = Person;
    fn add(self, rhs: Self) -> Self::Output {
        Person {
            name: format!("{}{}", self.name, rhs.name),
        }
    }
}

trait Printable {
    fn p(&self);
}

impl Printable for Person {
    fn p(&self) {
        println!("{:?}", self);
    }
}

// this is a static dispatch because which method to call is determined,
// staticaly, what happend is a seperate print_it_static is created for each type
// T is used
fn print_it_static<T>(z: T)
where
    T: Printable,
{
    z.p();
}

// opbject is dynamical called meaning which method it uses is determined at
// runtime, you would use dynamic messaageing is you had to go through an array
// of object and dynamicaly call there methonds.
// there is no why you could know at static time which method you would call.
fn print_it_dynamic(z: &dyn Printable) {
    z.p();
}

pub fn demo() {
    let rec: Rectangle = Shape::new(44.5, 66.7);
    let circle: Circle = Shape::new(22.2, 77.7);
    println!("{}{}", rec.length, rec.width);
    println!("{}{}", circle.length, circle.width);
    println!("{}", rec.area());
    println!("{}", circle.area());
    print_info(circle);

    let p = Person::new("Hello world");
    println!("{}", p.name);
    let p2 = Person::new("test");
    let p3 = p + p2;
    println!("{}", p3.name);
    print_it_static(p3);
    let p5 = Person::new("five");
    print_it_dynamic(&p5);

    // static dispatch: the decision of which method to call is determined
    // staticaly
}
