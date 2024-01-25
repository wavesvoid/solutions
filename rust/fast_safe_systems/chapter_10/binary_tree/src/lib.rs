/// Represents a binary tree structure.
/// 
/// It can be either [empty][BinaryTree::Empty] or [non-empty][nempty].
/// A [non-empty][nempty] tree holds a [TreeNode],
/// which in turn holds a value and two BinaryTree leafs.
///  
/// [nempty]: BinaryTree::NonEmpty
/// 
#[derive(Debug, PartialEq)]
pub enum BinaryTree<T: Ord> {
	Empty,
	NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    /// Create a new [BinaryTree::Empty]
    /// 
	pub fn new() -> Self {
		Self::Empty
	}
	
    /// Create a new, non-empty [binary tree][nemptree]
    /// with a [tree-node][node] given a specific value
    /// 
    /// [nemptree]: BinaryTree::NonEmpty
    /// [node]: TreeNode
    /// 
	pub fn with_value(value: T) -> Self {
		Self::NonEmpty(Box::new(TreeNode::new(value)))
	}

    /// Add a value to a binary tree
    /// 
    /// Adds a given value either to the left or right leaf.
    /// Depending on ordering of the values (greater go right, less go left)
    /// 
	pub fn add(&mut self, value: T) {
		match *self {
			BinaryTree::Empty => *self = BinaryTree::with_value(value),
			BinaryTree::NonEmpty(ref mut node) => node.append(value),
		}
	}

    pub fn left(&self) -> Option<&Self> {
        if let Self::NonEmpty(ref node) = self {
            return Some(&node.left);
        }
        None
    }

    pub fn right(&self) -> Option<&Self> {
        if let Self::NonEmpty(ref node) = self {
            return Some(&node.right);
        }
        None
    }
}


/// Node of the [binary tree][btree]
/// 
/// Represents a node in a binary tree, that contains a value of specific type.
/// And two leaf nodes, which are also [binary trees][btree].
/// A node has to contain a value that can be ordered
/// (to correctly respresent binary tree structure).
/// 
/// [btree]: BinaryTree
/// 
#[derive(Debug, PartialEq)]
pub struct TreeNode<T: Ord> {
	element: T,
	left: BinaryTree<T>,
	right: BinaryTree<T>,
}

impl<T: Ord> TreeNode<T> {
    /// Create an empty [TreeNode] with a given value.
    /// 
    /// The brand new node is created with empty [leafs][BinaryTree::Empty]
    /// 
	pub fn new(value: T) -> Self {
		Self {
			element: value,
			left: BinaryTree::new(),
			right: BinaryTree::new(),
		}
	}

    /// Append a [leaf][BinaryTree] with a given value to an existing node.
    /// 
    /// A comparsement is performed to determine which leaf append to.
    /// If the value is greater than current node's value then the right leaf.
    /// Otherwise - left.
    /// 
    /// A new [binary tree][BinaryTree::NonEmpty] will be put in place of the leaf.
    /// 
	pub fn append(&mut self, value: T) {
		if value <= self.element {
			self.left.add(value);
		} else {
			self.right.add(value);
		}
	}
}



#[cfg(test)]
mod tests {
    use super::*;


    mod utils {
        use super::{BinaryTree};
        use BinaryTree::*;


        /// Starting from the root tree
        /// traverse all the leafs adding them to a given stack.
        /// 
        /// Starting from the root tree node, start adding each leaf in order
        /// first left and then right leaf to the stack.
        /// Then move stack pointer to the next leaf (that was added to the stack)
        /// and expand its leafs to the stack.
        /// 
        /// Continue this loop until all leafs are traversed in the tree.
        /// At the finish the stack must contain all the tree leafs
        /// (including root tree as the first element) in order.
        /// 
        pub fn push_nonempty_leafs<'a, 'b: 'a, T: Ord>(
            root_tree: &'b BinaryTree<T>,
            stack: &'a mut Vec<&'b BinaryTree<T>>
        )
        {
            stack.push(&root_tree);

            // Traversing all nodes in the tree and flatten them into Vec
            //
            let mut root_needle = 0;
            loop {
                // Retrieve next root/leaf
                let Some(root @ NonEmpty(_)) =
                    stack.get(root_needle) else { break; };

                // This is irrefutable pattern,
                // because only non-empty elements are pushed to the stack
                let (Some(lleaf), Some(rleaf)) =
                    (root.left(), root.right()) else { unreachable!() };

                //
                match (lleaf, rleaf) {
                    (ref l @ NonEmpty(_), ref r @ NonEmpty(_)) => {
                        stack.push(l); stack.push(r);
                    }
                    (ref l @ NonEmpty(_), Empty) => stack.push(l),
                    (Empty, ref r @ NonEmpty(_)) => stack.push(r),
                    _ => { },
                };
                
                root_needle += 1; // move on to the next leaf
            }
        }
    }

    #[test]
    fn test_btree_empty() {
        let mut btree = BinaryTree::<i32>::new();
        assert_eq!(btree, BinaryTree::Empty);
    }

    #[test]
    fn test_btree_nonempty() {
        let btree = BinaryTree::with_value(5);

        let BinaryTree::NonEmpty(ref node) = btree else { panic!() };
        assert_eq!(node.element, 5);
    }

    #[test]
    fn test_btree_leafs() {
        let mut btree = BinaryTree::with_value(7);
        let elems    = [6, 8, 10, 7, 5, 1, 2, 0];
        let check_in = [7, 6, 8, 5, 7, 10, 1, 0, 2];
        
        // Concise adding to the tree (instead of manual duplication)
        elems.iter().for_each(|&el| btree.add(el));

        // Gather only non-empty leafs and store them in order as stack
        // -----
        let mut leaf_stack = Vec::with_capacity(check_in.len());
        utils::push_nonempty_leafs(&btree, &mut leaf_stack);

        // Verify the tree elements order
        // -----
        assert!(leaf_stack.len() == check_in.len());
        leaf_stack.iter()
            .zip(check_in)
            .for_each(|(bt, n): (&&BinaryTree<i32>, i32)| {
                // Irrefutable, but we need to decompose through a pattern
                let BinaryTree::NonEmpty(ref nt) = bt else { unreachable!() };
                assert!(nt.element == n);
            });
    }
}