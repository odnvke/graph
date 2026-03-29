use crate::{Graph, NodeIndex};


impl<T> Graph<T> {
    pub fn del_panic(&mut self, node_index: NodeIndex) {
        if !self.is_node_valid(&node_index) {
            panic!("\n  >>  addr {} invalid\n", node_index.index);
        }
        
        let idx = node_index.index;
        
        
        let links: Vec<NodeIndex> = 
            if idx < self.links.len() { self.links[idx].clone() } 
            else { Vec::new() };
        
        for neighbor_idx in links {     
            self.disconnect(neighbor_idx , node_index);
        }
        
        self.free.push(idx);
    }

    pub fn bfs_panic(&self, node_index: NodeIndex) -> Vec<NodeIndex>{
        if !self.is_node_valid(&node_index) {
            panic!("\n  >>  addr {} invalid\n", node_index.index)
        }
        
        use std::collections::VecDeque;
        use std::collections::HashSet;
        
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        visited.insert(node_index);
        queue.push_back(node_index);
        
        while let Some(node) = queue.pop_front() {
            result.push(node);
            
            // Получаем соседей
            let neighbors = self.get_links_from_node_panic(&node);
            
            for neighbor_idx in neighbors {
                let neighbor = neighbor_idx;
                
                if !visited.contains(&neighbor) {
                    visited.insert(*neighbor);
                    queue.push_back(*neighbor);
                }
            }
        }
        
        result
    }

    pub fn dfs_panic(&self, start: NodeIndex) -> Vec<NodeIndex> {
        if !self.is_node_valid(&start) {
            panic!("\n  >>  addr {} invalid\n", start.index);
        }
        
        use std::collections::HashSet;
        
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![start];
        
        while let Some(node) = stack.pop() {
            if !visited.contains(&node) {
                visited.insert(node);
                result.push(node);
                
                let neighbors = self.get_links_from_node_panic(&node);
                for &neighbor_idx in neighbors.iter().rev() {
                    let neighbor = neighbor_idx;
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        
        result
    }

    pub fn get_links_from_node_panic(&self, node_index: &NodeIndex) -> &Vec<NodeIndex> {
        if self.is_node_valid(node_index) {
            &self.links[node_index.index]
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }
    
    fn get_links_from_node_mut_panic(&mut self, node_index: &NodeIndex) -> &mut Vec<NodeIndex> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index { &mut self.links[node_index.index] } 
            else { panic!("\n  >>  no links with {} addr", node_index.index) }
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }

    fn is_connect_panic(&self, n_i: &NodeIndex, n2_i: &NodeIndex) -> bool {
        if !self.is_node_valid(n_i) { panic!("\n  >>  addr {} invalid\n", n_i.index) }
        if !self.is_node_valid(n2_i) { panic!("\n  >>  addr {} invalid\n", n2_i.index) }

        if self.links.len() <= n_i.index { false } 
        else if self.links[n_i.index].contains(&n2_i) { true }
        else { false } 
    }

    pub fn get_value_ref_panic(&self, node_index: NodeIndex) -> &T {
        if self.is_node_valid(&node_index) { &self.values[node_index.index]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }

    pub fn get_value_mut_ref_panic(&mut self, node_index: NodeIndex) -> &mut T {
        if self.is_node_valid(&node_index) { &mut self.values[node_index.index]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }


}