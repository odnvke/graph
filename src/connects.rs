use crate::{Graph, NodeIndex};

impl<T> Graph<T> {
    pub fn connect_all(&mut self, nodes: &[NodeIndex]) {

        for node in nodes {
            if !self.is_node_valid(node) {
                panic!("\n  >>  addr {} invalid\n", node.index)
            }
        }
        
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let from = nodes[i];
                let to = nodes[j];
                
                if !self.links[from.index].contains(&to) {
                    self.links[from.index].push(to);
                    self.links[to.index].push(from);
                }
            }
        }
    }

    pub fn loop_node(&mut self, nodes: &[NodeIndex]) {
        for node in nodes {
            if !self.is_node_valid(node) {
                panic!("\n  >>  addr {} invalid\n", node.index)
            }
        }
        
        for i in 0..nodes.len() {
            let from = nodes[i];
            let to = nodes[(i+1) % nodes.len()];

            if !self.links[from.index].contains(&to) {
                self.links[from.index].push(to);
                self.links[to.index].push(from);
            }
        }
    }
}