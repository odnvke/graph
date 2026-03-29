mod impl_;

use std::fmt::Debug;
//use connects;

fn main() {
    let mut g: Graph<i32> = Graph::new();

    let mut nodes = Vec::new();

    for i in 0..10 {
        nodes.push(g.get_new_node(i));
    }

    // g.connect(nodes[0], nodes[1]);

    g.loop_node(&nodes);

    println!("{:?}", g);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeIndex {
    index: usize
}

pub enum NodeError {
    InvalidIndex(NodeIndex),
    SelfLoop(NodeIndex),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::InvalidIndex(idx) => {
                write!(f, "invalid node index: {}", idx.index)
            }
            NodeError::SelfLoop(idx) => {
                write!(f, "self-loop not allowed at node {}", idx.index)
            }
        }
    }
}

pub struct Graph<T> {
    count: usize,
    values: Vec<T>,
    links: Vec<Vec<NodeIndex>>,
    free: Vec<usize>,
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for Graph<T> where T: std::fmt::Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut _str = String::new();
        
        for (i, elem) in self.links.iter().enumerate() {
            if !elem.is_empty() {
                _str.push_str(format!("  from: {}    to: ", i).as_str());
                for _s in elem {
                    _str.push_str(format!("{} ", _s.index).as_str());
                }
                _str.push('\n');
            }
        }

        write!(f, "\nGraph {{\n--------------\nnodes:\n{:#?}\n--------------\nedges:\n{}}}", self.values, _str)
    }
}

impl <T> Graph<T> where T: Eq {
    pub fn find(self, value: T) -> Option<NodeIndex> {
        self.iter_node()
            .find(|node_index| self.values[node_index.index] == value)
    }
}

impl <T> Graph<T> {
    pub fn new() -> Self {
        Self { count: 0, values: Vec::new(), links: Vec::new(), free: Vec::new() }
    }

    pub fn get_new_node(&mut self, value: T) -> NodeIndex {
        if self.free.is_empty() {
            self.values.push(value);
            self.links.push(Vec::new());
            self.count += 1;
            NodeIndex { index: self.values.len()-1}
        } else {
            let idx = self.free.pop().unwrap();
            self.values[idx] = value;
            NodeIndex { index: idx }
        }
    }


    pub fn iter_node(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        (0..self.values.len())
            .filter(|&i| !self.free.contains(&i))
            .map(|i| NodeIndex { index: i })
    }

    pub fn iter_value(&self) -> impl Iterator<Item = &T> + '_ {
        self.iter_node()
            .map(move |node| &self.values[node.index])
    }

    pub fn has_node(&self, node_index: &NodeIndex) -> bool {
        self.is_node_valid(node_index)
    }

    //###########################################################
    // ===  ===  ===  ===  ===  util func  ===  ===  ===  ===  ==
    //###########################################################
    pub fn first_node(&self) -> Option<NodeIndex> {
        if self.values.is_empty() { None } 
        else { Some(NodeIndex { index: 0 }) }
    }

    pub fn get_value_ref(&self, node_index: NodeIndex) -> Result<&T, NodeError> {
        if self.is_node_valid(&node_index) { 
            Ok(&self.values[node_index.index])
        } else { 
            Err(NodeError::InvalidIndex(node_index))
        }
    }

    pub fn get_value_mut_ref(&mut self, node_index: NodeIndex) -> Result<&mut T, NodeError> {
        if self.is_node_valid(&node_index) { 
            Ok(&mut self.values[node_index.index])
        } else { 
            Err(NodeError::InvalidIndex(node_index))
        }
    }

    //#[inline(always)]
    fn is_connect(&self, n_i: &NodeIndex, n2_i: &NodeIndex) -> Result<bool, NodeError> {
        if !self.is_node_valid(n_i) { return Err(NodeError::InvalidIndex(*n_i)); }
        if !self.is_node_valid(n2_i) { return Err(NodeError::InvalidIndex(*n2_i)); }

        Ok(self.links[n_i.index].contains(&n2_i))
    }

    //#[inline(always)]
    pub fn get_links_from_node(&self, node_index: &NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(node_index) { return Err(NodeError::InvalidIndex(*node_index)); } 

        Ok(&self.links[node_index.index])
    }

    //#[inline(always)]
    pub fn is_node_valid(&self, node_index: &NodeIndex) -> bool {
        if node_index.index >= self.count { false }
        else if self.free.contains(&node_index.index) { false }
        else { true }
    }
}
