use crate::{NodeIndex, NodeError};
use slotmap::{SlotMap, Key};
use std::fmt::{Debug};
use std::collections::{HashSet, VecDeque};

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

#[derive(Debug)]
pub struct NodeData<T, W> {
    pub value: T,
    pub links_in: Vec<(NodeIndex, W)>,
    pub links_out: Vec<(NodeIndex, W)>,
}

pub struct WUGraph<T> {
    pub nodes: SlotMap<NodeIndex, NodeData<T>>,
    allow_self_loop: bool,
    allow_pseudo_graph: bool
}

impl<T> Default for WUGraph<T, W> {
    fn default() -> Self {
        Self::new(false, false)
    }
}

impl<T> std::ops::Index<NodeIndex> for WUGraph<T> {
    type Output = T;
    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.value(index)
    }
}

impl<T> std::ops::IndexMut<NodeIndex> for WUGraph<T> {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        self.value_mut(index)
    }
}

impl<T: Debug> std::fmt::Debug for WUGraph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut edge_str = String::new();
        for (from_idx, data) in self.nodes.iter() {
            if !data.links_out.is_empty() {
                edge_str.push('\n');
                edge_str.push_str(&format!("    from: {:?}    to: ", from_idx.data()));
                for to_idx in &data.links_out {
                    edge_str.push_str(&format!("{:?}  ", to_idx.data()));
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
            "\nUGraph (self loop: {}, pseudo graph: {}):\n  nodes:{}\n  edges:{}",
            self.allow_self_loop,
            self.allow_pseudo_graph,
            vec_str,
            edge_str
        )
    }
}

impl<T: Eq> WUGraph<T> {
    pub fn find(&self, value: &T) -> Option<NodeIndex> {
        self.iter_node()
            .find(|&node| self.nodes[node].value == *value)
    }
}

impl<T> WUGraph<T> {
    pub fn new(allow_self_loop: bool, allow_pseudo_graph: bool) -> Self {
        Self {
            nodes: SlotMap::with_key(),
            allow_self_loop: allow_self_loop,
            allow_pseudo_graph: allow_pseudo_graph,
        }
    }

    pub fn new_node(&mut self, value: T) -> NodeIndex {
        self.nodes.insert(NodeData {
            value,
            links_in: Vec::new(),
            links_out: Vec::new(),
        })
    }

    // === Базовые методы ===
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        // Для ориентированного графа: сумма длин всех linksOut
        self.nodes.values().map(|data| data.links_out.len()).sum()
    }

    pub fn iter_node(&self) -> impl Iterator<Item = NodeIndex> + '_ {
        self.nodes.keys()
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (NodeIndex, NodeIndex)> + '_ {
        self.nodes.iter().flat_map(|(from, data)| {
            data.links_out.iter().map(move |&to| (from, to))
        })
    }

    pub fn iter_value(&self) -> impl Iterator<Item = &T> + '_ {
        self.nodes.values().map(|data| &data.value)
    }

    pub fn has_node(&self, node_index: NodeIndex) -> bool {
        self.nodes.contains_key(node_index)
    }

    pub fn is_allow_self_loop(&self) -> bool {
        self.allow_self_loop
    }

    pub fn is_pseudo_graph(&self) -> bool {
        self.allow_pseudo_graph
    }

    pub fn first_node(&self) -> Option<NodeIndex> {
        self.iter_node().next()
    }

    fn is_node_valid(&self, node: NodeIndex) -> bool {
        self.nodes.contains_key(node)
    }

    // === Neighbors (исходящие и входящие) ===
    pub fn try_neighbors_out(&self, node: NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        self.nodes.get(node).map(|data| &data.links_out).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_neighbors_in(&self, node: NodeIndex) -> Result<&Vec<NodeIndex>, NodeError> {
        self.nodes.get(node).map(|data| &data.links_in).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_neighbors_out_mut(&mut self, node: NodeIndex) -> Result<&mut Vec<NodeIndex>, NodeError> {
        self.nodes.get_mut(node).map(|data| &mut data.links_out).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_neighbors_in_mut(&mut self, node: NodeIndex) -> Result<&mut Vec<NodeIndex>, NodeError> {
        self.nodes.get_mut(node).map(|data| &mut data.links_in).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn neighbors_out(&self, node: NodeIndex) -> &Vec<NodeIndex> {
        match self.try_neighbors_out(node) {
            Ok(v) => v,
            Err(e) => panic!("neighbors_out failed: {}", e),
        }
    }

    pub fn neighbors_in(&self, node: NodeIndex) -> &Vec<NodeIndex> {
        match self.try_neighbors_in(node) {
            Ok(v) => v,
            Err(e) => panic!("neighbors_in failed: {}", e),
        }
    }

    pub fn try_iter_neighbors_out(&self, node: NodeIndex) -> Result<impl Iterator<Item = NodeIndex> + '_, NodeError> {
        Ok(self.try_neighbors_out(node)?.iter().copied())
    }

    pub fn try_iter_neighbors_in(&self, node: NodeIndex) -> Result<impl Iterator<Item = NodeIndex> + '_, NodeError> {
        Ok(self.try_neighbors_in(node)?.iter().copied())
    }

    pub fn iter_neighbors_out(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.neighbors_out(node).iter().copied()
    }

    pub fn iter_neighbors_in(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.neighbors_in(node).iter().copied()
    }

    // === Рёбра ===
    pub fn try_are_connected(&self, from_node: NodeIndex, to_node: NodeIndex) -> Result<bool, NodeError> {
        if !self.is_node_valid(from_node) {
            return Err(NodeError::InvalidIndex(from_node));
        }
        if !self.is_node_valid(to_node) {
            return Err(NodeError::InvalidIndex(to_node));
        }
        Ok(self.nodes[from_node].links_out.contains(&to_node))
    }

    pub fn try_edge_count_between(&self, from_node: NodeIndex, to_node: NodeIndex) -> Result<usize, NodeError> {
       if !self.is_node_valid(from_node) {
            return Err(NodeError::InvalidIndex(from_node));
        }
        if !self.is_node_valid(to_node) {
            return Err(NodeError::InvalidIndex(to_node));
        }
        Ok(self.nodes[from_node].links_out.iter().filter(|&&x| x == to_node).count())
    }

    pub fn try_connect(&mut self, from_node: NodeIndex, to_node: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(from_node) {
            return Err(NodeError::InvalidIndex(from_node));
        }
        if !self.is_node_valid(to_node) {
            return Err(NodeError::InvalidIndex(to_node));
        }
        if from_node == to_node && !self.allow_self_loop {
            return Err(NodeError::SelfLoop(from_node));
        }

        if !self.nodes[from_node].links_out.contains(&to_node) || self.allow_pseudo_graph {
            self.nodes[from_node].links_out.push(to_node);
            self.nodes[to_node].links_in.push(from_node);
        }
        Ok(())
    }

    pub fn try_disconnect(&mut self, from_node: NodeIndex, to_node: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(from_node) { return Err(NodeError::InvalidIndex(from_node)); }
        if !self.is_node_valid(to_node) { return Err(NodeError::InvalidIndex(to_node)); }

        // Удаляем все вхождения to из linksOut[from]
        let out = &mut self.nodes[from_node].links_out;
        let count_out = out.iter().filter(|&&x| x == to_node).count();
        out.retain(|&x| x != to_node);

        // Удаляем все вхождения from из linksIn[to]
        let inp = &mut self.nodes[to_node].links_in;
        let count_in = inp.iter().filter(|&&x| x == from_node).count();
        inp.retain(|&x| x != from_node);

        // Для консистентности проверяем, что оба счётчика равны (должны быть)
        let count = count_out.min(count_in);
        if count > 0 { Ok(()) } else { Err(NodeError::NoEdge(from_node, to_node)) }
    }

    pub fn try_remove_all_edges(&mut self, from_node: NodeIndex, to_node: NodeIndex) -> Result<usize, NodeError> {
        if !self.is_node_valid(from_node) { return Err(NodeError::InvalidIndex(from_node)); }
        if !self.is_node_valid(to_node) { return Err(NodeError::InvalidIndex(to_node)); }

        // Удаляем все вхождения to из linksOut[from]
        let out = &mut self.nodes[from_node].links_out;
        out.retain(|&x| x != to_node);

        // Удаляем все вхождения from из linksIn[to]
        let inp = &mut self.nodes[to_node].links_in;
        let mut count_in = inp.iter().filter(|&&x| x == from_node).count();
        inp.retain(|&x| x != from_node);

        if count_in > 0 { Ok(count_in) } else { Err(NodeError::NoEdge(from_node, to_node)) }
    }

    pub fn try_del(&mut self, node: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(node) {
            return Err(NodeError::InvalidIndex(node));
        }

        // Собираем уникальных соседей (все, кто есть в links_in или links_out)
        let mut neighbors = HashSet::new();
        neighbors.extend(&self.nodes[node].links_in);
        neighbors.extend(&self.nodes[node].links_out);

        // Удаляем все рёбра между node и каждым соседом в обе стороны
        for &neighbor in &neighbors {
            // из node в neighbor
            let _ = self.try_remove_all_edges(node, neighbor);
            // из neighbor в node
            let _ = self.try_remove_all_edges(neighbor, node);
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

    // === Обходы (по исходящим рёбрам) ===
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
            for &neighbor in &self.nodes[node].links_out {
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
                for &neighbor in self.nodes[node].links_out.iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        Ok(result)
    }

    // === value ===
    pub fn try_value(&self, node: NodeIndex) -> Result<&T, NodeError> {
        self.nodes.get(node).map(|data| &data.value).ok_or(NodeError::InvalidIndex(node))
    }

    pub fn try_value_mut(&mut self, node: NodeIndex) -> Result<&mut T, NodeError> {
        self.nodes.get_mut(node).map(|data| &mut data.value).ok_or(NodeError::InvalidIndex(node))
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

    // === connect_all, loop_node ===
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
                self.try_connect(nodes[j], nodes[i])?;
            }
        }
        Ok(())
    }

    // === Паникующие обёртки (используют макрос) ===
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

    pub fn are_connected(&self, from_node: NodeIndex, to_node: NodeIndex) -> bool {
        match self.try_are_connected(from_node, to_node) {
            Ok(v) => v,
            Err(e) => panic!("are_connected failed: {}", e),
        }
    }

    // Для обратной совместимости: если нужен один список (например, исходящие)
    pub fn neighbors(&self, node: NodeIndex) -> &Vec<NodeIndex> {
        self.neighbors_out(node)
    }

    pub fn iter_neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.iter_neighbors_out(node)
    }

    pub fn remove_all_edges(&mut self, from_node: NodeIndex, to_node: NodeIndex) -> usize {
        match self.try_remove_all_edges(from_node, to_node) {
            Ok(v) => v,
            Err(e) => panic!("remove_all_edges: {}", e),
        }
    }

    pub fn edge_count_between(&self, from_node: NodeIndex, to_node: NodeIndex) -> usize {
        match self.try_edge_count_between(from_node, to_node) {
            Ok(v) => v,
            Err(e) => panic!("edge_count_between failed: {}", e),
        }
    }
}