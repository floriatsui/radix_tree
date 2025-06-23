// creating a node struct to represent the tree
pub struct Node {
    // vector of addr or reference to children nodes
    pub child_nodes: Vec<Node>,

    // string of current value
    pub current_value: String,
}
