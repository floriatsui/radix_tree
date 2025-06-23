// this is where all of the radix tree operations will go
// like a node is a radix tree? so I feel like the node should be passed in here?
pub mod radix_tree {
    use crate::node::Node;

    // returns the number of characters that match
    // make sure that this accounts for the empty string case
    // this can also stay as a private function
    // we don't want ownership over these values, so this should be a borrow 
    fn prefix_check(curr_str: &String, check_str: &String) -> usize {
        let curr_str_bytes = curr_str.as_bytes(); 
        let check_str_bytes = check_str.as_bytes();
        let mut ptr = 0; 

        while ptr < curr_str.len() && ptr < check_str.len() && curr_str_bytes[ptr] == check_str_bytes[ptr] {
            ptr += 1 
        }
        
        // probably not the most efficient way to go about doing this, but oh well  
        return ptr;
    }

    // returns the initial root node
    pub fn radix_tree_init() -> Node {
        // TODO: this def is undefined behavior in C, not sure how this is handled in rust tbh
        return Node {
            child_nodes: Vec::new(),
            current_value: String::new(),
        };
    }

    // value may or may not eist
    pub fn radix_tree_lookup(root: Node, value: String) -> bool {
        // compare with the root node initially
        // if there's a prefi match then search children (this includes empty string I guess)
        // if not in children, then can terminate early.

        // keep searching the children while there's still a prefi match in the remaining
        // part of the string until the string is eahusted or the remainder doesn't have a match in the
        // list of children nodes
        let mut current_node: &Node = &root; 
        let result = prefix_check(&current_node.current_value, &value);
        // where the tree is completely empty
        if result == 0 && current_node.current_value.is_empty() && current_node.child_nodes.is_empty() {
            return false; 
        } else {
            let mut splits = value.split_at(result);
            let mut remainder = splits.1;
            while !remainder.is_empty() {
                // iterate through all of the child nodes
                for node in &(current_node.child_nodes) {
                    let result = prefix_check(&node.current_value, &remainder.to_string());
                    if result == remainder.len() {
                        // if the prefix check is equal to the reaminder then we can return 
                        return true; 
                    } else if result > 0 {
                        // there's a partial match 
                        // then we should continue this, but with the child node being the root 
                        current_node = &node; 
                        // update the new remainder to search the new root node's child 
                        splits = value.split_at(result); 
                        remainder = splits.1; 
                        break; 
                    }
                } 
            }
            return false;
        }
    }

    pub fn radix_tree_insert(root: Node, value: String) -> Node {
        if radix_tree_lookup(root, value) {
            // value may exist already
            return root;
        }

        // TODO: understand how the ownership works here
        let mut current_node: &Node = &root;
        // TODO: how do I add to a vector in rust?

        // the tree could be completely empty in which case the list of child nodes is empty
        // or there's only one node aka the root node
        let matched_characters: usize = prefix_check(&current_node.current_value, &value);
        // this is the case where the tree is completely empty to begin with
        if matched_characters == 0 && current_node.child_nodes.is_empty() {
            current_node.current_value = value;
            return *current_node;
        }

        // want to split in the matched characters here
        // there could be 0 or > 0

        // let's handle the 0 cse first
        if matched_characters == 0 && !current_node.child_nodes.is_empty() {
            // we should just add a new empty string root node
            // and insert the old root node as a child node
            let mut new_root_node = Node {
                child_nodes: Vec::new(),
                current_value: String::new(),
            };
            new_root_node.child_nodes.push(*current_node);
            return new_root_node;
        }
        // this means that there must've been some characters that were a match, aka the > 0 case
        // basically we need to determine if we need to split the root up
        // or if we can use the root's value as is
        let split_values = value.split_at(matched_characters);
        let prefix: String = split_values.0.to_string();
        let remain_value: String = split_values.1.to_string();

        while {
            if prefix == current_node.current_value {
                // where we need to keep going down the tree
                // iterate through the available child nodes, if there's a prefix match, aka the number of matched characters is
                // greater than 0, then set that as the current node. 
                
                // then we need to repeat this check here 
            } else {
                // we need to split the root node up
            }
            // this would be the matching prefix
        }
        return current_node;
    }

    // value also may not eist
    pub fn radix_tree_delete(root: Node, value: String) {
        // there's a notion of keeping track of the number of children
        // if the number of children goes down to one, make sure that it gets merged
        // it shouldn't have a chain reaction though since going up one level, there
        // shouldn't be a singular node at that level --> we should assert to make sure that is indeed the case
    }
}
