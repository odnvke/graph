mod impl_;
mod tests; // если есть

use std::fmt::{Debug, write};
use slotmap::{DefaultKey, Key, KeyData, SlotMap};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NodeIndex(DefaultKey);
impl std::fmt::Display for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.data()).trim_end_matches(|c: char| c.is_ascii_digit()).trim_end_matches('v'))
    }
}
impl std::fmt::Debug for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self.data()).trim_end_matches(|c: char| c.is_ascii_digit()).trim_end_matches('v'))
    }
}

unsafe impl Key for NodeIndex {
    fn data(&self) -> KeyData {
        self.0.data()
    }
}

impl From<KeyData> for NodeIndex {
    fn from(data: KeyData) -> Self {
        NodeIndex(DefaultKey::from(data))
    }
}

#[derive(Debug)]
pub enum NodeError {
    InvalidIndex(NodeIndex),
    SelfLoop(NodeIndex),
    NoEdge(NodeIndex, NodeIndex),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::InvalidIndex(idx) => write!(f, "invalid node index: {:?}", idx),
            NodeError::SelfLoop(idx) => write!(f, "self-loop not allowed at node {:?}", idx),
            NodeError::NoEdge(idx, idx2) => write!(f, "no edge between {:?} and {:?}", idx, idx2),
        }
    }
}

#[derive(Debug)]
struct NodeData<T> {
    value: T,
    links: Vec<NodeIndex>,
}

pub struct Graph<T> {
    nodes: SlotMap<NodeIndex, NodeData<T>>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> std::fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut edge_str = String::new();
        for (from_idx, data) in self.nodes.iter() {
            if !data.links.is_empty() {
                edge_str.push('\n');
                edge_str.push_str(&format!("    from: {:?}    to: ", from_idx.data()));
                for to_idx in &data.links {
                    edge_str.push_str(
                        &format!("{:?}  ", to_idx.data()));
                }
            }
        }

        let mut vec_str = String::new();
        let vec_nodes: Vec<NodeIndex> = self.iter_node().collect();
        let vec_values: Vec<&T> = self.iter_value().collect();
        assert_eq!(vec_nodes.len(), vec_values.len());
        for i in 0..vec_nodes.len() {
            vec_str.push('\n');
            vec_str.push_str(format!("    {:?}: {:?}", vec_nodes[i].data(), vec_values[i]).as_str());
        }
        write!(
            f,
            "\nGraph:\n  nodes:{}\n  edges:{}",
            vec_str,
            edge_str
        )
    }
}

impl<T: Eq> Graph<T> {
    pub fn find(&self, value: &T) -> Option<NodeIndex> {
        self.iter_node()
            .find(|&node| self.nodes[node].value == *value)
    }
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
        }
    }

    pub fn new_node(&mut self, value: T) -> NodeIndex {
        self.nodes.insert(NodeData {
            value,
            links: Vec::new(),
        })
    }
}

fn main() {
    let mut g: Graph<i32> = Graph::new();
    let node1 = g.new_node(10);
    let node2 = g.new_node(20);
    let node3 = g.new_node(30);
    let node4 = g.new_node(40);

    g.loop_node(vec![node1, node2, node3, node4]);

    println!("{:?}", g.find(&20));

    g.del(node2);

    g.disconnect(node2, node3);
    g.disconnect(node2, node3);

    println!("{:?}", g);
}