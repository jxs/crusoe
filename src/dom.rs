//! Basic DOM data structures.

use std::collections::hashmap::{HashMap, HashSet};

pub type AttrMap = HashMap<String, String>;

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {

    pub fn get_attribute<'a>(&'a self, key: &str) -> Option<&'a String> {
        self.attributes.find_equiv(&key)
    }
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

//constructors

impl Node {

    pub fn new_text_node(data: String) -> Node {
        Node{children: vec![], node_type: Text(data)}
    }

    pub fn new_elem_node(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node{
            children: children,
            node_type: Element(ElementData {
                tag_name: name,
                attributes: attrs,
            })
        }
    }
}
