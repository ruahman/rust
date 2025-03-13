// smart pointer are pointer with additional metadata and capabilities
// string and vector are smart pointers because they own the memory they point to
// and allow you to manipulate it and keep track of meta data.

// smart pointers are usually implemented as structs
// the implement the Deref and Drop traits

// it's a pointer that provides functionality beyond just access to a reference
// example a String can be consider a smart pointer

// rust needs to know how much space a type takes up at compile time

// Box<T> is a smart pointer because it implements the Deref trait, which allows Box<T> to be
// treated like a reference
// Box<T> also implements the Drop trait, which allows Box<T> to clean up the heap memory when it
// goes out of scope

#[derive(Debug)]
struct TreeNode<T> {
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
    pub key: T,
}

impl<T> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode {
            left: None,
            right: None,
            key,
        }
    }
    pub fn left(mut self, node: TreeNode<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    pub fn right(mut self, node: TreeNode<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

pub fn run() {
    // Box<T> is a type of smart pointer
    // Box<T> is a pointer that implements the Deref trait, which allows Box<T> to be treated like
    // a reference, the Deref trait allows you to use the deref operator * to dereference the box
    //
    //
    // Box<T> also implements the Drop trait, which allows Box<T> to clean up the heap memory when
    // it goes out of scope, when box goes out of scope the memory it points to in the heep will be
    // cleared
    let b_int = Box::new(10);
    // if you want the box to point to a type of trait it needs to be a dyn trait,
    // since it's imposible to know the size of the item in the heap by just a trait
    println!("{}", b_int);

    // Deref trait allows you to the deref operator such as & and *
    let y_box = Box::new(9);
    let y = 9;
    assert_eq!(y, *y_box);

    // the drop trait allow you to consimize what happens when a value goes out of scope

    let node1 = TreeNode::new(1);
    let x = node1.left(TreeNode::new(2)).right(TreeNode::new(3));
    println!("{:?}", x);
    println!("{}", x.key);
    // let node2 = TreeNode::new(2);
    // let node3 = TreeNode::new(3);

    // node1.left(node2);
    // node1.right(node3);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smart_pointers() {
        run()
    }
}
