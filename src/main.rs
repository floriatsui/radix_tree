// sample program using radix tree or just calling all of the tests or something
pub mod node;
pub mod radix_tree;

use crate::node::Node;
use crate::radix_tree::radix_tree_operations::radix_tree_init;

fn main() {
    println!("Hello, world!");
    let new_node: Node = radix_tree_init();
}
