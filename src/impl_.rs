use crate::{Graph, NodeIndex, NodeError};

// Только для простых случаев (без возврата)
macro_rules! panic_from_try {
    ($name:ident, $try_name:ident, $($arg:ident: $ty:ty),*) => {
        pub fn $name(&mut self, $($arg: $ty),*) {
            match self.$try_name($($arg),*) {
                Ok(()) => (),
                Err(e) => panic!(concat!(stringify!($name), " failed: {}"), e),
            }
        }
    };
}

impl<T> Graph<T> {
    // === TRY_ — Result для обработки ошибок ===

    pub fn try_are_connected(&self, node: &NodeIndex, node2: &NodeIndex) -> Result<bool, NodeError> {
        if !self.is_node_valid(node) { return Err(NodeError::InvalidIndex(*node)); }
        if !self.is_node_valid(node2) { return Err(NodeError::InvalidIndex(*node2)); }

        Ok(self.links[node.index].contains(&node2))
    }

    pub fn try_connect(&mut self, node: NodeIndex, node2: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&node) { return Err(NodeError::InvalidIndex(node)); }
        if !self.is_node_valid(&node2) { return Err(NodeError::InvalidIndex(node2)); }

        if node == node2 {
            return Err(NodeError::SelfLoop(node));
        }

        if !self.links[node.index].contains(&node2) {
            self.links[node.index].push(node2);
            self.links[node2.index].push(node);
        }

        Ok(())
    }

    pub fn try_disconnect(&mut self, node: NodeIndex, node2: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&node) { return Err(NodeError::InvalidIndex(node)); }
        if !self.is_node_valid(&node2) { return Err(NodeError::InvalidIndex(node2)); }

        if let Some(pos_a) = self.links[node2.index].iter().position(|&x| x == node) {
            let pos_b = self.links[node.index].iter().position(|&x| x == node2).unwrap();
            self.links[node2.index].swap_remove(pos_a);
            self.links[node.index].swap_remove(pos_b);
            Ok(())
        } else {
            Err(NodeError::NoEdge(node, node2))
        }
    }

    pub fn try_del(&mut self, node: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }

        let links: Vec<NodeIndex> = self.links[node.index].clone();
        for neighbor in links {
            self.try_disconnect(neighbor, node)?;
        }

        self.free.push(node.index);
        Ok(())
    }

    pub fn try_insert(&mut self, node: NodeIndex) -> Result<T, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }

        let links: Vec<NodeIndex> = self.links[node.index].clone();
        for neighbor in links {
            self.try_disconnect(neighbor, node)?;
        }

        self.free.push(node.index);
        Ok(self.values.insert(node.index, ))
    }

    pub fn try_bfs(&self, node: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }

        use std::collections::{VecDeque, HashSet};

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(node);
        queue.push_back(node);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            for &neighbor in self.try_links(node)? {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        Ok(result)
    }

    pub fn try_dfs(&self, node: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }

        use std::collections::HashSet;

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![node];

        while let Some(node) = stack.pop() {
            if !visited.contains(&node) {
                visited.insert(node);
                result.push(node);
                for &neighbor in self.try_links(node)?.iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn try_value(&self, node: NodeIndex) -> Result<&T, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }
        Ok(&self.values[node.index])
    }

    pub fn try_value_mut(&mut self, node: NodeIndex) -> Result<&mut T, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }
        Ok(&mut self.values[node.index])
    }

    pub fn try_links(&self, node: NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }
        Ok(&self.links[node.index])
    }

    pub fn try_links_mut(&mut self, node: NodeIndex) -> Result<&mut Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node) {
            return Err(NodeError::InvalidIndex(node));
        }
        Ok(&mut self.links[node.index])
    }

    pub fn try_connect_all<I>(&mut self, nodes: I) -> Result<(), NodeError> 
    where 
        I: IntoIterator<Item = NodeIndex>,
    {
        let nodes: Vec<_> = nodes.into_iter().collect();
        
        for node in &nodes {
            if !self.is_node_valid(node) {
                return Err(NodeError::InvalidIndex(*node));
            }
        }

        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                self.try_connect(nodes[i], nodes[j])?;
            }
        }

        Ok(())
    }

    pub fn try_loop_node<I>(&mut self, nodes: I) -> Result<(), NodeError> 
    where 
        I: IntoIterator<Item = NodeIndex>,
    {
        let nodes: Vec<_> = nodes.into_iter().collect();
        for node in &nodes {
            if !self.is_node_valid(node) {
                return Err(NodeError::InvalidIndex(*node));
            }
        }

        for i in 0..nodes.len() {
            let from = nodes[i];
            let to = nodes[(i + 1) % nodes.len()];
            self.try_connect(from, to)?;
        }

        Ok(())
    }

    // === КОРОТКИЕ — паника при ошибке (генерируются макросом) ===

    panic_from_try!(connect, try_connect, a: NodeIndex, b: NodeIndex);
    panic_from_try!(disconnect, try_disconnect, a: NodeIndex, b: NodeIndex);
    panic_from_try!(del, try_del, idx: NodeIndex);

    pub fn connect_all<I>(&mut self, nodes: I) where I: IntoIterator<Item = NodeIndex> {
        match self.try_connect_all(nodes) {
            Ok(()) => (),
            Err(e) => panic!("connect_all failed: {}", e),
        }
    }

    pub fn loop_node<I>(&mut self, nodes: I) where I: IntoIterator<Item = NodeIndex> {
        match self.try_loop_node(nodes) {
            Ok(()) => (),
            Err(e) => panic!("loop_node failed: {}", e),
        }
    }

    pub fn bfs(&self, node: NodeIndex) -> Vec<NodeIndex> {
        match self.try_bfs(node) {
            Ok(v) => v,
            Err(e) => panic!("bfs failed: {}", e),
        }
    }

    pub fn dfs(&self, node: NodeIndex) -> Vec<NodeIndex> {
        match self.try_dfs(node) {
            Ok(v) => v,
            Err(e) => panic!("dfs failed: {}", e),
        }
    }

    pub fn value(&self, node: NodeIndex) -> &T {
        match self.try_value(node) {
            Ok(v) => v,
            Err(e) => panic!("value failed: {}", e),
        }
    }

    pub fn value_mut(&mut self, node: NodeIndex) -> &mut T {
        match self.try_value_mut(node) {
            Ok(v) => v,
            Err(e) => panic!("value_mut failed: {}", e),
        }
    }

    pub fn links(&self, node: NodeIndex) -> &Vec<NodeIndex> {
        match self.try_links(node) {
            Ok(v) => v,
            Err(e) => panic!("links failed: {}", e),
        }
    }

    pub fn links_mut(&mut self, node: NodeIndex) -> &mut Vec<NodeIndex> {
        match self.try_links_mut(node) {
            Ok(v) => v,
            Err(e) => panic!("links_mut failed: {}", e),
        }
    }

    pub fn are_connected(&self, node: &NodeIndex, node2: &NodeIndex) -> bool {
        match self.try_are_connected(node, node2) {
            Ok(v) => v,
            Err(e) => panic!("is_connect failed: {}", e),
        }
    }
}