mod connects;

use std::fmt::Debug;
//use connects;

fn main() {
    let mut g: Graph<i32> = Graph::new();

    let mut nodes = Vec::new();

    for i in 0..10 {
        nodes.push(g.get_new_node(i));
    }

    g.connect(nodes[0], nodes[1]);

    g.loop_node(&nodes);

    println!("{:?}", g);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NodeIndex {
    index: usize
}

pub enum NodeError {
    InvalidIndex(NodeIndex),
    AlreadyConnect(NodeIndex, NodeIndex),
    AlreadyDisconnect(NodeIndex, NodeIndex),

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

    pub fn connect(&mut self, n_i: NodeIndex, n2_i: NodeIndex) -> Result<(), NodeError>{
        if !self.is_node_valid(&n_i) { return Err(NodeError::InvalidIndex(n_i)); }
        if !self.is_node_valid(&n2_i) { return Err(NodeError::InvalidIndex(n2_i)); }

        if self.is_connect_panic(&n_i, &n2_i) { Err(NodeError::AlreadyConnect(n_i, n2_i)) }
        else {
            self.links[n_i.index].push(n2_i);
            self.links[n2_i.index].push(n_i);

            Ok(())
        }
    }
    
    pub fn disconnect(&mut self, n_i: NodeIndex, n2_i: NodeIndex) -> Result<(), NodeError>{
        if !self.is_node_valid(&n_i) { return Err(NodeError::InvalidIndex(n_i)); }
        if !self.is_node_valid(&n2_i) { return Err(NodeError::InvalidIndex(n2_i)); }

        if !self.is_connect_panic(&n_i, &n2_i) { Err(NodeError::AlreadyDisconnect(n_i, n2_i)) }
        else {       
            let pos_in_min = self.links[n2_i.index].iter().position(|link| *link == n_i).unwrap();
            let pos_in_max = self.links[n_i.index].iter().position(|link| *link == n2_i).unwrap();
            
            self.links[n2_i.index].swap_remove(pos_in_min);
            self.links[n_i.index].swap_remove(pos_in_max);
        
        Ok(())
        }
    }

    pub fn del(&mut self, node_index: NodeIndex) -> Result<(), NodeError> {
        if !self.is_node_valid(&node_index) {
            panic!("\n  >>  addr {} invalid\n", node_index.index);
        }
        
        let idx = node_index.index;
        
        
        let links: Vec<NodeIndex> = 
            if idx < self.links.len() { self.links[idx].clone() } 
            else { Vec::new() };
        
        for neighbor_idx in links {     
            self.disconnect(neighbor_idx , node_index)?;
        }
        
        self.free.push(idx);
    }

    pub fn bfs(&self, start: NodeIndex) -> Vec<NodeIndex> {
        if !self.is_node_valid(&start) {
            panic!("\n  >>  addr {} invalid\n", start.index);
        }
        
        use std::collections::VecDeque;
        use std::collections::HashSet;
        
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        visited.insert(start);
        queue.push_back(start);
        
        while let Some(node) = queue.pop_front() {
            result.push(node);
            
            // Получаем соседей
            let neighbors = self.get_links_from_node_panic(&node);
            
            for &neighbor_idx in neighbors {
                let neighbor = neighbor_idx;
                
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        
        result
    }
    
    pub fn dfs(&self, start: NodeIndex) -> Vec<NodeIndex> {
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
                
                // Получаем соседей в обратном порядке для сохранения порядка
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

    pub fn get_value_ref(&self, node_index: NodeIndex) -> &T {
        if self.is_node_valid(&node_index) { &self.values[node_index.index]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }

    pub fn get_value_mut_ref(&mut self, node_index: NodeIndex) -> &mut T {
        if self.is_node_valid(&node_index) { &mut self.values[node_index.index]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }
    
    //#[inline(always)]
    fn get_links_from_node_panic(&self, node_index: &NodeIndex) -> &Vec<NodeIndex> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index { &self.links[node_index.index] } 
            else { panic!("\n  >>  no links with {} addr", node_index.index) }
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }

    //#[inline(always)]
    fn get_links_from_node_panic_mut(&mut self, node_index: &NodeIndex) -> &mut Vec<NodeIndex> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index { &mut self.links[node_index.index] } 
            else { panic!("\n  >>  no links with {} addr", node_index.index) }
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index) }
    }

    //#[inline(always)]
    fn is_connect_panic(&self, n_i: &NodeIndex, n2_i: &NodeIndex) -> bool {
        if !self.is_node_valid(n_i) { panic!("\n  >>  addr {} invalid\n", n_i.index) }
        if !self.is_node_valid(n2_i) { panic!("\n  >>  addr {} invalid\n", n2_i.index) }

        if self.links.len() <= n_i.index { false } 
        else if self.links[n_i.index].contains(&n2_i) { true }
        else { false } 
    }

    //#[inline(always)]
    pub fn get_links_from_node(&mut self, node_index: &NodeIndex) -> Option<&Vec<NodeIndex>> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index { Some(&self.links[node_index.index]) } 
            else { None }
        } 
        else { None }
    }

    //#[inline(always)]
    pub fn is_node_valid(&self, node_index: &NodeIndex) -> bool {
        if node_index.index >= self.count { false }
        else if self.free.contains(&node_index.index) { false }
        else { true }
    }
}



















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph: Graph<i32> = Graph::new();
        assert_eq!(graph.count, 0);
        assert!(graph.values.is_empty());
        assert!(graph.links.is_empty());
        assert!(graph.free.is_empty());
    }

    #[test]
    fn test_get_new_node() {
        let mut graph = Graph::new();
        
        let node1 = graph.get_new_node(42);
        assert_eq!(graph.count, 1);
        assert_eq!(graph.values.len(), 1);
        assert_eq!(*graph.get_value_ref(node1), 42);

        let node2 = graph.get_new_node(100);
        assert_eq!(graph.count, 2);
        assert_eq!(graph.values.len(), 2);
        assert_eq!(*graph.get_value_ref(node2), 100);
    }

    #[test]
    fn test_get_first_node() {
        let mut graph = Graph::new();
        
        // Пустой граф
        assert!(graph.first_node().is_none());
        
        // С узлами
        graph.get_new_node(42);
        let first = graph.first_node().unwrap();
        assert_eq!(*graph.get_value_ref(first), 42);
    }

    #[test]
    fn test_get_value_ref() {
        let mut graph = Graph::new();
        let node = graph.get_new_node(42);
        
        assert_eq!(*graph.get_value_ref(node), 42);
    }

    #[test]
    fn test_get_value_mut_ref() {
        let mut graph = Graph::new();
        let node = graph.get_new_node(42);
        
        *graph.get_value_mut_ref(node) = 100;
        assert_eq!(*graph.get_value_ref(node), 100);
    }

    #[test]
    fn test_is_node_valid() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let node2 = graph.get_new_node(100);
        
        assert!(graph.is_node_valid(&node1));
        assert!(graph.is_node_valid(&node2));
        
        // Невалидный индекс
        let invalid_node = NodeIndex { index: 999 };
        assert!(!graph.is_node_valid(&invalid_node));
    }

    #[test]
    fn test_connect() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let node2 = graph.get_new_node(100);
        let node3 = graph.get_new_node(200);
        
        graph.connect(node1, node2);
        
        // Проверяем, что связи добавились
        assert!(graph.links[node1.index].contains(&node2));
        assert!(graph.links[node2.index].contains(&node1));
        
        // Добавляем еще одну связь
        graph.connect(node1, node3);
        assert!(graph.links[node1.index].contains(&node3));
        assert!(graph.links[node3.index].contains(&node1));
    }

    #[test]
    #[should_panic(expected = "addr 999 invalid")]
    fn test_connect_invalid_node() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let invalid_node = NodeIndex { index: 999 };
        
        graph.connect(node1, invalid_node);
    }

    #[test]
    #[should_panic(expected = "addr 0 already connect with 1")]
    fn test_connect_duplicate() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let node2 = graph.get_new_node(100);
        
        graph.connect(node1, node2);
        graph.connect(node1, node2); // Должно вызвать панику
    }

    #[test]
    fn test_disconnect() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let node2 = graph.get_new_node(100);
        
        graph.connect(node1, node2);
        assert!(graph.links[node1.index].contains(&node2));
        
        graph.disconnect(node1, node2);
        assert!(!graph.links[node1.index].contains(&node2));
        assert!(!graph.links[node2.index].contains(&node1));
    }

    #[test]
    #[should_panic(expected = "addr 0 not connect with any node")]
    fn test_disconnect_not_connected() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let node2 = graph.get_new_node(100);
        
        graph.disconnect(node1, node2); // Должно вызвать панику
    }

    #[test]
    fn test_free_list_reuse() {
        let mut _graph: Graph<i32> = Graph::new();
        
        // Создаем узел и удаляем его (но в нашей реализации нет удаления)
        // Для теста reuse мы можем использовать тот факт, что free не пуст только при удалении
        // Но в текущей реализации нет удаления узлов, поэтому этот тест будет пропущен
        // или нужно добавить метод remove_node
    }

    #[test]
    fn test_complex_operations() {
        let mut graph = Graph::new();
        
        // Создаем несколько узлов
        let nodes: Vec<NodeIndex> = (0..5).map(|i| graph.get_new_node(i)).collect();
        
        // Создаем полный граф (каждый с каждым)
        for i in 0..nodes.len() {
            for j in i+1..nodes.len() {
                graph.connect(nodes[i], nodes[j]);
            }
        }
        
        // Проверяем все связи
        for i in 0..nodes.len() {
            assert_eq!(graph.links[nodes[i].index].len(), nodes.len() - 1);
            for j in 0..nodes.len() {
                if i != j {
                    assert!(graph.links[nodes[i].index].contains(&nodes[j]));
                }
            }
        }
        
        // Удаляем несколько связей
        graph.disconnect(nodes[0], nodes[1]);
        graph.disconnect(nodes[0], nodes[2]);
        
        // Проверяем результат
        assert!(!graph.links[nodes[0].index].contains(&nodes[1]));
        assert!(!graph.links[nodes[1].index].contains(&nodes[0]));
        assert!(!graph.links[nodes[0].index].contains(&nodes[2]));
        assert!(!graph.links[nodes[2].index].contains(&nodes[0]));
        
        // Проверяем остальные связи
        assert!(graph.links[nodes[0].index].contains(&nodes[3]));
        assert!(graph.links[nodes[3].index].contains(&nodes[0]));
    }
}