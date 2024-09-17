use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;

// a trait is a collection of methods that an object must implement
// in order to be classified as implementing that trait

// think of a trait as an interface in other languages
// traits only consists of method signatures
// in order to implement a trait you have to implement all the methods
trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

// a derivable trait is a trait that is automatically implemented by the compiler
// for a struct or enum
// #[derive(Debug)] allows you to print the struct or enum
// #[derive(clone)] allows you to clone the struct or enum
// #[derive(copy)] allows you to copy the struct or enum

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Dog {
    name: &'static str,
}

// you can also specify the trait of your generic type.
// make sure that the generic type implements the traits you specify
// Output is an associated type an specifies the return type of the function
#[allow(dead_code)]
fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

struct Human {
    name: &'static str,
}

// once human implements the animal trait, it becomes an animal
impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

enum Creature {
    Human(Human),
    Cat(Cat),
}

trait Summable<T> {
    fn sum(&self) -> T;
}

// implement summable for Vec<i32>
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

// this is a shap trait
trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32,
}

#[derive(Debug)]
struct Circle {
    length: f32,
    width: f32,
}

#[allow(clippy::approx_constant)]
const PI: f32 = 3.14;

// implement the shape trait for the rectangle
impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
        Rectangle { length, width }
    }
    fn area(&self) -> f32 {
        self.length * self.width
    }
}

// implement the shape trait for the circle
impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
        Circle { length, width }
    }
    fn area(&self) -> f32 {
        (self.length / 2.0).powf(2.0) * PI
    }
}

// take in parameter that implements the shape trait
// and the debug trait sot that we can print it
fn print_info(shape: impl Shape + Debug) {
    println!("the area is {}", shape.area());
}

// you can also use a generics to define the trait parameter
fn print_info2<T: Shape + Debug>(shape: T) {
    println!("{}", shape.area())
}

// another way to define a generic type for the trait parameter
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

// Into allows automatic conversion of a type into another whenever possible
// Into<String> makes sure that the name can be converted to a string
impl Person {
    fn new<S>(name: S) -> Person
    where
        S: Into<String>,
    {
        // into() that the implementation of into to convert the name to a string if it is not
        // into() will convert a str to a String
        Person { name: name.into() }
    }
}

// Drop notifies you when an object goes out of scope and is being deallocated
impl Drop for Person {
    fn drop(&mut self) {
        println!("drop person");
    }
}

// implement the add trait for the person
// this is how you can overload the + operator
impl Add for Person {
    // you have to specify the output type if you want to overload the + operator
    type Output = Person;

    fn add(self, rhs: Self) -> Self::Output {
        Person {
            name: format!("{}{}", self.name, rhs.name),
        }
    }
}

fn notify(p: &impl Shape) {
    println!("{}", p.area());
}

// this also works
fn notify2<T: Shape>(p: &T) {
    println!("{}", p.area());
}

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

// overload the add operator for the complex type
// this can take in any type that implements the add trait
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    // to overide add you need to specify the output type
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
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

// this is a static dispatch because which method to call is already determined staticaly,
// durring compile time
// staticaly, what happend is a seperate print_it_static is created for each type
// T is used
fn print_it_static<T>(z: T)
where
    T: Printable,
{
    // which implementation of p to use is determined at compile time
    z.p();
}

// however, if you make a function that uses a trait as a parameter,
// you need to specify that the trait is dynamic.
// opbject is dynamical called meaning which method it uses is determined at
// runtime, you would use dynamic messaageing is you had to go through an array
// of object and dynamicaly call there methonds.
// there is no why you could know at static time which method you would call.
// at run time it needs to consult a table to list what functions are available
// for the referenced object,

fn print_it_dynamic(z: &dyn Printable) {
    // &dyn specifies that this is a dynamic refrence
    // which implementation of p to use is determined at runtime
    z.p();
}

// trait objects:
// if a function returns a trait it needs to know the exact size of the return type
// at compile time. This is not possible with traits because they are dynamic.

// you can't return a trait object from a function
// you can only return a reference to a trait
// but you have to specify it as a Box<dyn Trait>
// so that the compiler knows that the method calls will be resolved at runtime

#[allow(clippy::vec_init_then_push)]
pub fn run() {
    // let h = Human { name: "John" };
    // let h = Human::create("John");
    let h: Human = Animal::create("John");
    // since human implements the animal trait, it can use the talk method
    h.talk();

    let mut creatures: Vec<Creature> = Vec::new();
    creatures.push(Creature::Human(Human::create("John")));
    creatures.push(Creature::Cat(Cat::create("Whiskers")));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    // this wont work because the animal trait size is not known at compile time
    // let animals: Vec<Animal> = Vec::new();

    // creatures.push(Box::new(Human::create("John")));
    // creatures.push(Cat::create("Whiskers"));

    let a = vec![1, 2, 3];
    // vec<i32> implements the summable trait here
    println!("{}", a.sum());

    let rec: Rectangle = Shape::new(44.5, 66.7);
    let circle: Circle = Shape::new(22.2, 77.7);
    println!("{}{}", rec.length, rec.width);
    println!("{}{}", circle.length, circle.width);
    println!("{}", rec.area());
    println!("{}", circle.area());
    println!("notify: {:?}", { notify(&rec) });
    println!("notify2: {:?}", { notify2(&rec) });
    // this function takes in a parameter that implements the shape trait and the debug trait
    print_info(circle);
    print_info(rec);
    let circle: Circle = Shape::new(22.2, 77.7);
    print_info2(circle);
    let rec: Rectangle = Shape::new(44.5, 66.7);
    print_info3(rec);

    let a = Complex::new(3, 4);
    let b = Complex::new(5, 6);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", a + b);

    // Into<String> makes sure that the name can be converted to a string
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

    struct Sedan {}

    impl LandCapable for Sedan {
        fn drive(&self) {
            println!("sedan driving");
        }
    }

    #[allow(dead_code)]
    struct Suv {}
    impl LandCapable for Suv {
        fn drive(&self) {
            println!("SUV driving");
        }
    }

    trait LandCapable {
        fn drive(&self);
    }

    // this is dynamic dispatch because the method to call is determined at runtime
    // which make the program size smaller
    // but there is a runtime cost
    fn road_trip_dyn(car: &dyn LandCapable) {
        car.drive();
    }

    // this is static dispatch because the method to call is determined at compile time
    // an implementation for each posible type is created for the function at compile time,
    // which makes the program size bigger
    fn road_trip_static<T: LandCapable>(car: &T) {
        car.drive();
    }

    // impl is another way to specify static dispatch
    fn road_trip_static2(car: &impl LandCapable) {
        car.drive();
    }

    let car = Sedan {};
    road_trip_dyn(&car);
    road_trip_static(&car);
    road_trip_static2(&car);

    // generics and structs usually work hand in hand
}

// cargo test variables::tests -- --nocapture
#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_traits() {
        run()
    }
}
