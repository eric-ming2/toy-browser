use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Node {
    pub(crate) children: Vec<Node>,
    node_type: NodeType,
}

impl Node {
    fn to_string(&self) -> String {
        match &self.node_type {
            NodeType::Text(text) => {
                format!("Text({:?})", text)
            }
            NodeType::Element(data) => {
                format!("Element({:?})", data)
            }
        }
    }
    fn pretty_print(&self, prefix: String, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}├──{:?}\n", prefix, self.to_string()).expect("Failed to write");
        for (_, child) in self.children.iter().enumerate() {
            let next_prefix = format!("{}   ", prefix);
            child.pretty_print(next_prefix, f).expect("Failed to write");
        }
        Ok(())
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pretty_print("".to_string(), f)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.node_type != other.node_type {
            return false;
        }
        if self.children.len() != other.children.len() {
            return false;
        }
        for i in 0..self.children.len() {
            if self.children[i] != other.children[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for Node {}

#[derive(PartialEq, Eq)]
enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(PartialEq, Eq)]
struct ElementData {
    tag_name: String,
    attrs: AttrMap,
}

impl Debug for ElementData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "tag_name: {}, attrs: {:?}", self.tag_name, self.attrs)
    }
}

#[derive(PartialEq, Eq)]
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
