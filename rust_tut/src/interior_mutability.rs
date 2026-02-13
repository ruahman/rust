#![allow(dead_code)]
// the ablility to mutate data through an imutable refrence
// you can have one mutable borrows
// or you can have multible imutable borrows
// but not at the same time
// these rules are inforced in compile time
// but
// some times you need to be able to mutate data behind an imutable reference
// that is where refcell and interior mutability come in
// refcell inforces borrowing ruels at run time
// this is really useful with structs
// refcell only works with single thread
// if you want interior mutability in multiple threads then use
// mutex or RwLock
// use Cell for copy types
// use RefCell for types that can be copied
use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Node<'a> {
    val: Cell<i32>,
    val_string: RefCell<String>,
    // only one thread can modify the data at any time
    adjacent: Vec<&'a Node<'a>>,
}

fn add_one(node: &Node) {
    let curr_val = node.val.get();
    let mut curr_string = node.val_string.borrow_mut();
    curr_string.push_str(" refcell");

    node.val.set(curr_val + 1);
    for adj in node.adjacent.iter() {
        add_one(adj);
    }
}

pub fn internal_mutability() {
    println!("internal_mutability");
    let data: RefCell<i32> = RefCell::new(5);
    println!("data: {}", data.borrow());

    // get a mutable refrenece in run time
    *data.borrow_mut() += 10;
    println!("data: {}", data.borrow());

    let a = Node {
        val: Cell::new(1),
        val_string: RefCell::new("one".to_string()),
        adjacent: vec![],
    };

    let b = Node {
        val: Cell::new(2),
        val_string: RefCell::new("two".to_string()),
        adjacent: vec![&a],
    };

    let c = Node {
        val: Cell::new(3),
        val_string: RefCell::new("three".to_string()),
        adjacent: vec![&a],
    };

    dbg!(&b);
    add_one(&b);
    dbg!(&b);
    dbg!(&c);
    add_one(&c);
    dbg!(&c);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_internal_mutability() {
        internal_mutability();
    }
}
