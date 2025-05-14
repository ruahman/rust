#![allow(dead_code)]

// you have to create an iterator and then run it.

// an iterator is just a object the iterates through a collection
pub fn iterators() {
    let vec = vec![1, 2, 3, 4, 5];

    // don't do this it will move vec and you cant use it again???
    // for x in vec {
    //     println!("{}", x);
    // }

    // this is better
    for x in &vec {
        println!("&{}", x);
    }

    // this is better also
    for x in vec.iter() {
        println!("{}", x);
    }

    // reverse
    for x in vec.iter().rev() {
        println!("{}", x);
    }

    println!("vec: {:?}", vec);

    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.extend(vec);

    // you move element from vec to vec2,
    // so you can't use vec again
    println!("vec2: {:?}", vec2);

    // this will cause problems
    // println!("vec: {:?}", vec);

    let foo = vec![1, 2, 3];
    // map just creates an iterator
    // collect launches the iterator
    // iter does not consume your collection
    let foo2: Vec<_> = foo.iter().map(|x| x + 1).collect();

    dbg!(&foo2);

    // into_iter consumes your collection and moves it to another datatype
    let foo3: Vec<_> = vec![1, 2, 3].into_iter().map(|x| x * x).collect();
    dbg!(&foo3);

    let foo4: Vec<_> = vec![1, 2, 3]
        .iter()
        .enumerate()
        .map(|(i, v)| i * v)
        .collect();

    dbg!(&foo4);

    struct RectIter {
        points: Vec<(f64, f64)>,
        idx: usize,
    }

    #[derive(Debug)]
    struct Rect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    // iterator for rec
    impl Iterator for RectIter {
        type Item = (f64, f64);

        fn next(&mut self) -> Option<Self::Item> {
            if self.idx >= self.points.len() {
                return None;
            }

            let point = self.points[self.idx];
            self.idx += 1;
            Some(point)
        }
    }

    // generate iterator for &Rect so that iterator does not cosume rect
    impl IntoIterator for &Rect {
        type Item = (f64, f64);
        type IntoIter = RectIter;

        fn into_iter(self) -> Self::IntoIter {
            return RectIter {
                idx: 0,
                points: vec![
                    (self.x, self.y),
                    (self.x + self.width, self.y),
                    (self.x, self.y + self.height),
                    (self.x + self.width, self.y + self.height),
                ],
            };
        }
    }

    impl Default for Rect {
        fn default() -> Self {
            Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            }
        }
    }

    let rect = Rect::default();

    for point in &rect {
        dbg!(point);
    }
    dbg!(rect);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iterators() {
        iterators();
    }
}
