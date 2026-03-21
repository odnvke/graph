fn main() {
    let mut g: Graph<i32> = Graph::new();

    let node0 = g.get_new_node(10);
    let node1 = g.get_new_node(20);
    let node2 = g.get_new_node(20);

    g.connect(node0, node1);
    g.connect(node2, node1);
    g.connect(node2, node0);

    println!("{:?}", g.get_links_from_node_panic(&node0));
    println!("{:?}", g.get_links_from_node_panic(&node1));

    g.del(node0);
}

struct Graph<T> {
    count: usize,
    values: Vec<T>,
    links: Vec<Vec<usize>>,
    free: Vec<usize>,
}

impl <T> Graph<T> {
    pub fn new() -> Self {
        Self { count: 0, values: Vec::new(), links: Vec::new(), free: Vec::new() }
    }

    pub fn get_new_node(&mut self, value: T) -> NodeIndex {
        if self.free.is_empty() {
            self.values.push(value);
            self.count += 1;
            NodeIndex { index_: self.values.len()-1}
        } else {
            let idx = self.free.pop().unwrap();
            self.values[idx] = value;
            NodeIndex { index_: idx }
        }
    } 

    pub fn connect(&mut self, n_i: NodeIndex, n2_i: NodeIndex) {
        if !self.is_node_valid(&n_i) {panic!("\n  >>  addr {} invalid\n", n_i.index_)}
        if !self.is_node_valid(&n2_i) {panic!("\n  >>  addr {} invalid\n", n2_i.index_)}
        let n = n_i.index_;
        let n2 = n2_i.index_;
        loop { 
            if n < self.links.len() && n2 < self.links.len() {
                if self.links[n].contains(&n2) { panic!("\naddr {} already connect with {}\n", n, n2) }
                break;
            } else {
                self.links.push(Vec::new());
            }
        }

        self.links[n].push(n2);
        self.links[n2].push(n);
    }

    pub fn disconnect(&mut self, n_i: NodeIndex, n2_i: NodeIndex) {
        if !self.is_node_valid(&n_i) { panic!("\n  >>  addr {} invalid\n", n_i.index_) }
        if !self.is_node_valid(&n2_i) { panic!("\n  >>  addr {} invalid\n", n2_i.index_) }

        if !self.is_connect_panic(&n_i, &n2_i) { panic!("\n  >>  addr {} already not connect with {}", n_i.index_, n2_i.index_) }

        let n = n_i.index_;
        let n2 = n2_i.index_;

        let pos_in_min = self.links[n2].iter().position(|link| *link == n).unwrap();
        let pos_in_max = self.links[n].iter().position(|link| *link == n2).unwrap();
        
        self.links[n2].swap_remove(pos_in_min);
        self.links[n].swap_remove(pos_in_max);
    }

    pub fn del(&mut self, node_index: NodeIndex) {
        if !self.is_node_valid(&node_index) { panic!("\n  >>  addr {} invalid\n", node_index.index_) }
        let mut positions = self.get_links_from_node_panic_mut(&node_index);

        loop {
            let pos = positions.pop().unwrap() else { break };

            
        }
    }



    //###########################################################
    // ===  ===  ===  ===  ===  util func  ===  ===  ===  ===  ==
    //###########################################################
    pub fn get_first_node(&self) -> Option<NodeIndex> {
        if self.values.is_empty() { None } 
        else { Some(NodeIndex { index_: 0 }) }
    }

    pub fn get_value_ref(&self, node_index: NodeIndex) -> &T {
        if self.is_node_valid(&node_index) { &self.values[node_index.index_]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index_) }
    }

    pub fn get_value_mut_ref(&mut self, node_index: NodeIndex) -> &mut T {
        if self.is_node_valid(&node_index) { &mut self.values[node_index.index_]}
        else { panic!("\n  >>  addr {} invalid\n", node_index.index_) }
    }
    
    //#[inline(always)]
    fn get_links_from_node_panic(&mut self, node_index: &NodeIndex) -> &Vec<usize> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index_ { &self.links[node_index.index_] } 
            else { panic!("\n  >>  no links with {} addr", node_index.index_) }
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index_) }
    }

    //#[inline(always)]
    fn get_links_from_node_panic_mut(&mut self, node_index: &NodeIndex) -> &mut Vec<usize> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index_ { &mut self.links[node_index.index_] } 
            else { panic!("\n  >>  no links with {} addr", node_index.index_) }
        } 
        else { panic!("\n  >>  addr {} invalid\n", node_index.index_) }
    }

    //#[inline(always)]
    fn is_connect_panic(&mut self, n_i: &NodeIndex, n2_i: &NodeIndex) -> bool {
        if !self.is_node_valid(n_i) { panic!("\n  >>  addr {} invalid\n", n_i.index_) }
        if !self.is_node_valid(n2_i) { panic!("\n  >>  addr {} invalid\n", n2_i.index_) }

        if self.links.len() <= n_i.index_ { false } 
        else if self.links[n_i.index_].contains(&n2_i.index_) { true }
        else { false } 
    }

    //#[inline(always)]
    pub fn get_links_from_node(&mut self, node_index: &NodeIndex) -> Option<&Vec<usize>> {
        if self.is_node_valid(node_index) {
            if self.links.len() > node_index.index_ { Some(&mut self.links[node_index.index_]) } 
            else { None }
        } 
        else { None }
    }

    //#[inline(always)]
    pub fn is_node_valid(&self, node_index: &NodeIndex) -> bool {
        if node_index.index_ >= self.count { false }
        else if self.free.contains(&node_index.index_) { false }
        else { true }
    }
}

#[derive(Debug, Clone, Copy)]
struct NodeIndex {
    index_: usize
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
        assert!(graph.get_first_node().is_none());
        
        // С узлами
        graph.get_new_node(42);
        let first = graph.get_first_node().unwrap();
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
        let invalid_node = NodeIndex { index_: 999 };
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
        assert!(graph.links[node1.index_].contains(&node2.index_));
        assert!(graph.links[node2.index_].contains(&node1.index_));
        
        // Добавляем еще одну связь
        graph.connect(node1, node3);
        assert!(graph.links[node1.index_].contains(&node3.index_));
        assert!(graph.links[node3.index_].contains(&node1.index_));
    }

    #[test]
    #[should_panic(expected = "addr 999 invalid")]
    fn test_connect_invalid_node() {
        let mut graph = Graph::new();
        let node1 = graph.get_new_node(42);
        let invalid_node = NodeIndex { index_: 999 };
        
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
        assert!(graph.links[node1.index_].contains(&node2.index_));
        
        graph.disconnect(node1, node2);
        assert!(!graph.links[node1.index_].contains(&node2.index_));
        assert!(!graph.links[node2.index_].contains(&node1.index_));
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
            assert_eq!(graph.links[nodes[i].index_].len(), nodes.len() - 1);
            for j in 0..nodes.len() {
                if i != j {
                    assert!(graph.links[nodes[i].index_].contains(&nodes[j].index_));
                }
            }
        }
        
        // Удаляем несколько связей
        graph.disconnect(nodes[0], nodes[1]);
        graph.disconnect(nodes[0], nodes[2]);
        
        // Проверяем результат
        assert!(!graph.links[nodes[0].index_].contains(&nodes[1].index_));
        assert!(!graph.links[nodes[1].index_].contains(&nodes[0].index_));
        assert!(!graph.links[nodes[0].index_].contains(&nodes[2].index_));
        assert!(!graph.links[nodes[2].index_].contains(&nodes[0].index_));
        
        // Проверяем остальные связи
        assert!(graph.links[nodes[0].index_].contains(&nodes[3].index_));
        assert!(graph.links[nodes[3].index_].contains(&nodes[0].index_));
    }
}