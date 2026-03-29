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

    pub fn try_connect(&mut self, a: NodeIndex, b: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&a) { return Err(NodeError::InvalidIndex(a)); }
        if !self.is_node_valid(&b) { return Err(NodeError::InvalidIndex(b)); }

        if a == b {
            return Err(NodeError::SelfLoop(a));
        }

        self.links[a.index].push(b);
        self.links[b.index].push(a);
        Ok(())
    }

    pub fn try_disconnect(&mut self, a: NodeIndex, b: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&a) { return Err(NodeError::InvalidIndex(a)); }
        if !self.is_node_valid(&b) { return Err(NodeError::InvalidIndex(b)); }

        let pos_a = self.links[b.index].iter().position(|&x| x == a).unwrap();
        let pos_b = self.links[a.index].iter().position(|&x| x == b).unwrap();

        self.links[b.index].swap_remove(pos_a);
        self.links[a.index].swap_remove(pos_b);
        Ok(())
    }

    pub fn try_del(&mut self, node_index: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }

        let links: Vec<NodeIndex> = self.links[node_index.index].clone();
        for neighbor in links {
            self.try_disconnect(neighbor, node_index)?;
        }

        self.free.push(node_index.index);
        Ok(())
    }

    pub fn try_bfs(&self, node_index: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }

        use std::collections::{VecDeque, HashSet};

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(node_index);
        queue.push_back(node_index);

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

    pub fn try_dfs(&self, start: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&start) {
            return Err(NodeError::InvalidIndex(start));
        }

        use std::collections::HashSet;

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![start];

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

    pub fn try_value(&self, node_index: NodeIndex) -> Result<&T, NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }
        Ok(&self.values[node_index.index])
    }

    pub fn try_value_mut(&mut self, node_index: NodeIndex) -> Result<&mut T, NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }
        Ok(&mut self.values[node_index.index])
    }

    pub fn try_links(&self, node_index: NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }
        Ok(&self.links[node_index.index])
    }

    pub fn try_links_mut(&mut self, node_index: NodeIndex) -> Result<&mut Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(&node_index) {
            return Err(NodeError::InvalidIndex(node_index));
        }
        Ok(&mut self.links[node_index.index])
    }

    pub fn try_connect_all(&mut self, nodes: &[NodeIndex]) -> Result<(), NodeError> {
        for node in nodes {
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

    pub fn try_loop_node(&mut self, nodes: &[NodeIndex]) -> Result<(), NodeError> {
        for node in nodes {
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
    panic_from_try!(connect_all, try_connect_all, nodes: &[NodeIndex]);
    panic_from_try!(loop_node, try_loop_node, nodes: &[NodeIndex]);

        pub fn bfs(&self, start: NodeIndex) -> Vec<NodeIndex> {
        match self.try_bfs(start) {
            Ok(v) => v,
            Err(e) => panic!("bfs failed: {}", e),
        }
    }

    pub fn dfs(&self, start: NodeIndex) -> Vec<NodeIndex> {
        match self.try_dfs(start) {
            Ok(v) => v,
            Err(e) => panic!("dfs failed: {}", e),
        }
    }

    pub fn value(&self, idx: NodeIndex) -> &T {
        match self.try_value(idx) {
            Ok(v) => v,
            Err(e) => panic!("value failed: {}", e),
        }
    }

    pub fn value_mut(&mut self, idx: NodeIndex) -> &mut T {
        match self.try_value_mut(idx) {
            Ok(v) => v,
            Err(e) => panic!("value_mut failed: {}", e),
        }
    }

    pub fn links(&self, idx: NodeIndex) -> &Vec<NodeIndex> {
        match self.try_links(idx) {
            Ok(v) => v,
            Err(e) => panic!("links failed: {}", e),
        }
    }

    pub fn links_mut(&mut self, idx: NodeIndex) -> &mut Vec<NodeIndex> {
        match self.try_links_mut(idx) {
            Ok(v) => v,
            Err(e) => panic!("links_mut failed: {}", e),
        }
    }
}