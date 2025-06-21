// this is where all of the radix tree operations will go
// like a node is a radix tree? so I feel like the node should be passed in here?
pub mod radix_tree {
    use crate::node::Node;

    // returns the number of characters that match
    pub fn prefix_check() -> i32 {
        return 0;
    }

    // returns the initial root node
    pub fn radix_tree_init() -> Node {
        return Node {
            child_nodes: Vec::new(),
            current_value: String::new(),
        };
    }

    // value may or may not eist
    pub fn radix_tree_lookup(root: Node, value: String) -> bool {
        return false;
    }

    // value may eist already
    pub fn radix_tree_insert(root: Node, value: String) {
        let current_node: Node = root;
        // creating a special case initially
        // TODO: how do I add to a vector in rust?
        if current_node.child_nodes.is_empty() {
            current_node.child_nodes.append(other);
            return;
        } else {
            // check to see the prefi match with the contents of the root node
            // if there is a prefi match,
            // then check to see if an eisitng match is there
            // if not, insert it and return

            // if there is, then we should set that node as the current node
            // assess whether or not there's a prefi match and continue
            // we should put this in a while loop
            while {
                
            }
        }
    }

    // value also may not eist
    pub fn radix_tree_delete(root: Node, value: String) {}
}
