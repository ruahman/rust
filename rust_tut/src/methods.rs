#![allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}
pub fn run() {
    println!("---methods---");
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let my_line = Line { start: p1, end: p2 };
    println!("length: {}", my_line.length());
}

impl Line {
    fn length(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods() {
        run();
    }
}
