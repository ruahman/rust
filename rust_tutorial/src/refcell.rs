// refcells inforce the borrow rules at runtime instead of compile time

// you need to get around the borrow checker sometimes.

// interior mutability pattern

// you can mutate the value in a refcell even if there are multiple refrerences to it.

// you use refcell when you need to get around the borrow checker at compile time.
// but you still use the borrow checker at runtime.

// at any one time you can have one mutable reference
// or multiple immutable references.
// but not both.

// Rc has multiple owners pointing to the same data.
//
// RefCell allows borrow checks at runtime.

pub fn run() {
    println!("RefCell");
    use std::cell::{Cell, RefCell};

    #[derive(Debug)]
    struct Node<'a> {
        value: Cell<i32>,
        value2: RefCell<String>,
        adjacent: Vec<&'a Node<'a>>,
    }

    fn add_one(node: &Node) {
        // node can't be a mutable reference because other nodes may have a reference to it.

        // can now change property without using a mutable reference for node.
        let cur_value = node.value.get();
        node.value.set(cur_value + 1);

        let mut mut_value = node.value2.borrow_mut();
        mut_value.push('x');

        // some of the adjacent nodes may have a reference to this node.
        // use iter so that n does not take ownership of the reference.
        for n in node.adjacent.iter() {
            add_one(n);
        }
    }

    let a = Node {
        value: Cell::new(1),
        value2: RefCell::new(String::from("one")),
        adjacent: vec![],
    };

    let b = Node {
        value: Cell::new(2),
        value2: RefCell::new(String::from("two")),
        adjacent: vec![&a],
    };

    let c = Node {
        value: Cell::new(3),
        value2: RefCell::new(String::from("three")),
        adjacent: vec![&a],
    };

    add_one(&b);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_refcell() {
        run();
    }
}
