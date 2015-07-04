//! Basic DOM data structures

use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive (Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive (Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

#[derive (Debug)]
pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}


pub fn text(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Text(data) }
}

pub fn comment(data: String) -> Node {
    Node { children: vec![], node_type: NodeType::Comment(data) }
}


pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
