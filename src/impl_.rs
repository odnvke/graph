use crate::{Graph, NodeIndex, NodeError};
use std::collections::{VecDeque, HashSet};
use slotmap::SlotMap;

// Макрос для генерации паникующих обёрток
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
    // === TRY_ — Result-версии ===    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.nodes
            .values()
            .map(|data| data.links.len())
            .sum::<usize>() / 2
    }

    pub fn iter_node(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.nodes.keys()
    }

    pub fn iter_value(&self) -> impl Iterator<Item = &T> + '_ {
        self.nodes.values().map(|data| &data.value)
    }

    pub fn has_node(&self, node_index: NodeIndex) -> bool {
        self.nodes.contains_key(node_index)
    }

    pub fn try_iter_neighbors(&self, node: NodeIndex) -> Result<impl Iterator<Item = NodeIndex> + '_, NodeError> {
        Ok(self.try_links(node)?.iter().copied())
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (NodeIndex, NodeIndex)> + '_ {
        self.iter_node().flat_map(move |from| {
            self.iter_neighbors(from)
                .filter(move |&to| from < to)
                .map(move |to| (from, to))
        })
    }

    pub fn first_node(&self) -> Option<NodeIndex> {
        self.iter_node().next()
    }

    fn is_node_valid(&self, node: NodeIndex) -> bool {
        self.nodes.contains_key(node)
    }

    pub fn try_are_connected(&self, node: NodeIndex, node2: NodeIndex) -> Result<bool, NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }
        if !self.is_node_valid(node2) {
            return Err(NodeError::InvalidIndex(node2));
        }
        Ok(self.nodes[node].links.contains(&node2))
    }

    pub fn try_connect(&mut self, node: NodeIndex, node2: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }
        if !self.is_node_valid(node2) {
            return Err(NodeError::InvalidIndex(node2));
        }
        if node == node2 {
            return Err(NodeError::SelfLoop(node));
        }

        if !self.nodes[node].links.contains(&node2) {
            self.nodes[node].links.push(node2);
            self.nodes[node2].links.push(node);
        }
        Ok(())
    }

    pub fn try_disconnect(&mut self, node: NodeIndex, node2: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }
        if !self.is_node_valid(node2) {
            return Err(NodeError::InvalidIndex(node2));
        }

        let removed_from_node2 = if let Some(pos) = self.nodes[node2].links.iter().position(|&x| x == node) {
            self.nodes[node2].links.swap_remove(pos);
            true
        } else {
            false
        };

        let removed_from_node = if let Some(pos) = self.nodes[node].links.iter().position(|&x| x == node2) {
            self.nodes[node].links.swap_remove(pos);
            true
        } else {
            false
        };

        if removed_from_node && removed_from_node2 {
            Ok(())
        } else {
            Err(NodeError::NoEdge(node, node2))
        }
    }

    pub fn try_del(&mut self, node: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }

        let neighbors: Vec<NodeIndex> = self.nodes[node].links.clone();
        for neighbor in neighbors {
            self.try_disconnect(neighbor, node)?;
        }
        self.nodes.remove(node);
        Ok(())
    }

    pub fn try_insert(&mut self, node: NodeIndex, new_value: T) -> Result<T, NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }
        let old_value = std::mem::replace(&mut self.nodes[node].value, new_value);
        Ok(old_value)
    }

    pub fn try_bfs(&self, start: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(start) {
            return Err(NodeError::InvalidIndex(start));
        }

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            for &neighbor in &self.nodes[node].links {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        Ok(result)
    }

    pub fn try_dfs(&self, start: NodeIndex) -> Result<Vec<NodeIndex>, NodeError> {
        if !self.is_node_valid(start) {
            return Err(NodeError::InvalidIndex(start));
        }

        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            if !visited.contains(&node) {
                visited.insert(node);
                result.push(node);
                for &neighbor in self.nodes[node].links.iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn try_value(&self, node: NodeIndex) -> Result<&T, NodeError> {
        self.nodes.get(node).map(|data| &data.value).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_value_mut(&mut self, node: NodeIndex) -> Result<&mut T, NodeError> {
        self.nodes.get_mut(node).map(|data| &mut data.value).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_links(&self, node: NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        self.nodes.get(node).map(|data| &data.links).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_links_mut(&mut self, node: NodeIndex) -> Result<&mut Vec<NodeIndex>, NodeError> {
        self.nodes.get_mut(node).map(|data| &mut data.links).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_connect_all<I>(&mut self, nodes: I) -> Result<(), NodeError>
    where
        I: IntoIterator<Item = NodeIndex>,
    {
        let nodes: Vec<_> = nodes.into_iter().collect();
        for &node in &nodes {
            if !self.is_node_valid(node) {
                return Err(NodeError::InvalidIndex(node));
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
        for &node in &nodes {
            if !self.is_node_valid(node) {
                return Err(NodeError::InvalidIndex(node));
            }
        }
        for i in 0..nodes.len() {
            let from = nodes[i];
            let to = nodes[(i + 1) % nodes.len()];
            self.try_connect(from, to)?;
        }
        Ok(())
    }

    // === Короткие паникующие версии ===

    panic_from_try!(connect, try_connect, a: NodeIndex, b: NodeIndex);
    panic_from_try!(disconnect, try_disconnect, a: NodeIndex, b: NodeIndex);
    panic_from_try!(del, try_del, idx: NodeIndex);

    pub fn connect_all<I>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = NodeIndex>,
    {
        match self.try_connect_all(nodes) {
            Ok(()) => (),
            Err(e) => panic!("connect_all failed: {}", e),
        }
    }

    pub fn loop_node<I>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = NodeIndex>,
    {
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

    pub fn are_connected(&self, node: NodeIndex, node2: NodeIndex) -> bool {
        match self.try_are_connected(node, node2) {
            Ok(v) => v,
            Err(e) => panic!("are_connected failed: {}", e),
        }
    }

    pub fn iter_neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        match self.try_iter_neighbors(node) {
            Ok(v) => v,
            Err(e) => panic!("iter_neighbors failed: {}", e),
        }
    }
}