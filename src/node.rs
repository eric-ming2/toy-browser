use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Node {
    pub(crate) children: Vec<Node>,
    node_type: NodeType,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node_type {
            NodeType::Text(text) => {
                write!(f, "Text({:?})", text)
            }
            NodeType::Element(data) => {
                write!(f, "Element({:?})", data)
            }
        }
    }
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

impl Debug for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tag_name: {}, attrs: {:?}", self.tag_name, self.attrs)
    }
}

pub(crate) struct AttrMap {
    pub(crate) attrs: HashMap<String, String>,
}

impl Debug for AttrMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.attrs.iter() {
            write!(f, "{}: {}, ", key, value).expect("Failed to write to formatter");
        }
        Ok(())
    }
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs }),
    }
}

pub fn pretty_print(node: &Node, prefix: String) {
    println!("{}├──{:?}", prefix, node);
    for (_, child) in node.children.iter().enumerate() {
        let next_prefix = format!("{}   ", prefix);
        pretty_print(child, next_prefix);
    }
}
