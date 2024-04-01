pub fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut res = Vec::new();
    let mut q = Vec::new();
    q.push(root);

    while !q.is_empty() {
        let node = q.remove(0);
        res.push(node.value);

        if let Some(left) = &node.left {
            q.push(left);
        }

        if let Some(right) = &node.right {
            q.push(right);
        }
    }

    res
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(
            tree_by_levels(&Node::new(42)),
            [42],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
        assert_eq!(
            tree_by_levels(&root),
            [1, 2, 3, 4, 5, 6],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }
}

#[derive(Debug)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn left(mut self, left: Node) -> Self {
        self.left = Some(Box::new(left));
        self
    }

    fn right(mut self, right: Node) -> Self {
        self.right = Some(Box::new(right));
        self
    }
}
