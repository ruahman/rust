// it's a pointer that provides functionality beyond just access to a reference
// example a String can be consider a smart pointer

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
    let b_int = Box::new(10);
    println!("{}", b_int);

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
