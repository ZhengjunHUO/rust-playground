pub enum BinaryTree<T> {
    NoTree,
    Tree(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    content: T,
    left_child: BinaryTree<T>,
    right_child: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn append(&mut self, content: T) {
        match *self {
            BinaryTree::NoTree => {
                *self = BinaryTree::Tree(Box::new(TreeNode {
                    content,
                    left_child: BinaryTree::NoTree,
                    right_child: BinaryTree::NoTree,
                }))
            }
            BinaryTree::Tree(ref mut node) => {
                if node.content >= content {
                    node.left_child.append(content);
                } else {
                    node.right_child.append(content);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let mut bt = BinaryTree::NoTree;
        bt.append(25);
        assert!(matches!(bt, BinaryTree::Tree(..)));
        bt.append(12);
        bt.append(69);

        match bt {
            BinaryTree::Tree(ref node) => {
                assert_eq!(node.content, 25);
            }
            _ => {}
        }
    }
}
