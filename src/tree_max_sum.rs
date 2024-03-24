fn max_sum(tree: Option<&TreeNode>) -> i32 {
    match tree {
        Some(n) => {
            let mut sum = n.value;
            let max_left = match &n.left {
                Some(i) => Some(max_sum(Some(&i))),
                _ => None,
            };
            let max_right = match &n.right {
                Some(i) => Some(max_sum(Some(&i))),
                _ => None,
            };

            sum += match (max_left, max_right) {
                (Some(left), Some(right)) => {
                    if left > right {
                        left
                    } else {
                        right
                    }
                }
                (Some(left), None) => left,
                (None, Some(right)) => right,
                (None, None) => 0,
            };

            sum
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::{max_sum, TreeNode};

    fn do_test(tree: Option<TreeNode>, expected: i32) {
        let actual = max_sum(tree.as_ref());
        assert!(actual == expected,
            "Test failed.\n\n   expected: {expected}\ninstead got: {actual}\n\nfor the tree:\n{tree:#?}\n\n")
    }

    #[test]
    fn empty_tree() {
        do_test(None, 0);
    }

    #[test]
    fn perfect_tree() {
        //      5
        //    /   \
        //  -22    11
        //  / \    / \
        // 9  50  9   2
        let tree = TreeNode::new(5)
            .with_left(
                TreeNode::new(-22)
                    .with_left(TreeNode::new(9))
                    .with_right(TreeNode::new(50)),
            )
            .with_right(
                TreeNode::new(11)
                    .with_left(TreeNode::new(9))
                    .with_right(TreeNode::new(2)),
            );
        do_test(Some(tree), 33)
    }

    #[test]
    fn only_stops_at_leaves() {
        //         5
        //       /   \
        //    4        10
        //   /  \     /
        // -80  -60 -90
        let tree = TreeNode::new(5)
            .with_left(
                TreeNode::new(4)
                    .with_left(TreeNode::new(-80))
                    .with_right(TreeNode::new(-60)),
            )
            .with_right(TreeNode::new(10).with_left(TreeNode::new(-90)));
        do_test(Some(tree), -51);
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn with_left(self, node: TreeNode) -> Self {
        TreeNode {
            left: Some(Box::new(node)),
            ..self
        }
    }

    fn with_right(self, node: TreeNode) -> Self {
        TreeNode {
            right: Some(Box::new(node)),
            ..self
        }
    }
}
