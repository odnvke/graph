use crate::{Graph, NodeIndex, NodeError};
use slotmap::Key; // позволяет использовать Key::null() для невалидных ключей

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_graph() {
        let graph: Graph<i32> = Graph::new();
        assert_eq!(graph.node_count(), 0);
    }

    #[test]
    fn test_get_new_node() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        assert_eq!(graph.node_count(), 1);
        assert_eq!(*graph.try_value(node1).unwrap(), 42);

        let node2 = graph.new_node(100);
        assert_eq!(graph.node_count(), 2);
        assert_eq!(*graph.try_value(node2).unwrap(), 100);
    }

    #[test]
    fn test_first_node() {
        let mut graph = Graph::new();
        assert!(graph.first_node().is_none());

        graph.new_node(42);
        let first = graph.first_node().unwrap();
        assert_eq!(*graph.try_value(first).unwrap(), 42);
    }

    #[test]
    fn test_get_value_ref() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);
        assert_eq!(*graph.try_value(node).unwrap(), 42);

        let invalid = Key::null(); // невалидный ключ
        assert!(matches!(graph.try_value(invalid), Err(NodeError::InvalidIndex(_))));
    }

    #[test]
    fn test_get_value_mut_ref() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);

        *graph.try_value_mut(node).unwrap() = 100;
        assert_eq!(*graph.try_value(node).unwrap(), 100);
    }

    #[test]
    fn test_is_node_valid() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let node2 = graph.new_node(100);

        assert!(graph.has_node(node1));
        assert!(graph.has_node(node2));

        let invalid_node = Key::null();
        assert!(!graph.has_node(invalid_node));
    }

    // === Тесты try_ методов ===

    #[test]
    fn test_try_connect() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let node2 = graph.new_node(100);

        assert!(graph.try_connect(node1, node2).is_ok());
        assert!(graph.links(node1).contains(&node2));
        assert!(graph.links(node2).contains(&node1));
    }

    #[test]
    fn test_try_connect_invalid_node() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let invalid_node = Key::null();

        assert!(matches!(
            graph.try_connect(node1, invalid_node),
            Err(NodeError::InvalidIndex(_))
        ));
    }

    #[test]
    fn test_try_connect_self_loop() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);

        assert!(matches!(
            graph.try_connect(node, node),
            Err(NodeError::SelfLoop(_))
        ));
    }

    #[test]
    fn test_try_disconnect() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let node2 = graph.new_node(100);

        graph.try_connect(node1, node2).unwrap();
        assert!(graph.links(node1).contains(&node2));

        assert!(graph.try_disconnect(node1, node2).is_ok());
        assert!(!graph.links(node1).contains(&node2));
        assert!(!graph.links(node2).contains(&node1));
    }

    #[test]
    fn test_try_del() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let node2 = graph.new_node(100);
        let node3 = graph.new_node(200);

        graph.try_connect(node1, node2).unwrap();
        graph.try_connect(node2, node3).unwrap();

        assert!(graph.try_del(node2).is_ok());
        assert!(!graph.has_node(node2));
        assert!(!graph.links(node1).contains(&node2));
        assert!(!graph.links(node3).contains(&node2));
    }

    #[test]
    fn test_try_bfs() {
        let mut graph = Graph::new();
        let n0 = graph.new_node(0);
        let n1 = graph.new_node(1);
        let n2 = graph.new_node(2);
        let n3 = graph.new_node(3);

        graph.try_connect(n0, n1).unwrap();
        graph.try_connect(n1, n2).unwrap();
        graph.try_connect(n0, n3).unwrap();

        let bfs_result = graph.try_bfs(n0).unwrap();
        assert_eq!(bfs_result.len(), 4);
        assert_eq!(bfs_result[0], n0);
    }

    #[test]
    fn test_try_dfs() {
        let mut graph = Graph::new();
        let n0 = graph.new_node(0);
        let n1 = graph.new_node(1);
        let n2 = graph.new_node(2);

        graph.try_connect(n0, n1).unwrap();
        graph.try_connect(n1, n2).unwrap();

        let dfs_result = graph.try_dfs(n0).unwrap();
        assert_eq!(dfs_result.len(), 3);
        assert!(dfs_result.contains(&n0));
        assert!(dfs_result.contains(&n1));
        assert!(dfs_result.contains(&n2));
    }

    #[test]
    fn test_try_value_and_links() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);

        assert_eq!(*graph.try_value(node).unwrap(), 42);
        assert!(graph.try_links(node).unwrap().is_empty());

        *graph.try_value_mut(node).unwrap() = 100;
        assert_eq!(*graph.try_value(node).unwrap(), 100);
    }

    #[test]
    fn test_try_connect_all() {
        let mut graph = Graph::new();
        let nodes: Vec<NodeIndex> = (0..4).map(|i| graph.new_node(i)).collect();

        assert!(graph.try_connect_all(nodes.iter().copied()).is_ok());

        for &node in &nodes {
            assert_eq!(graph.links(node).len(), nodes.len() - 1);
        }
    }

    #[test]
    fn test_try_loop_node() {
        let mut graph = Graph::new();
        let nodes: Vec<NodeIndex> = (0..4).map(|i| graph.new_node(i)).collect();

        assert!(graph.try_loop_node(nodes.iter().copied()).is_ok());

        assert!(graph.links(nodes[0]).contains(&nodes[1]));
        assert!(graph.links(nodes[1]).contains(&nodes[2]));
        assert!(graph.links(nodes[2]).contains(&nodes[3]));
        assert!(graph.links(nodes[3]).contains(&nodes[0]));
    }

    // === Тесты паникующих методов ===

    #[test]
    #[should_panic(expected = "connect failed")]
    fn test_connect_panic_invalid() {
        let mut graph = Graph::new();
        let node1 = graph.new_node(42);
        let invalid_node = Key::null();

        graph.connect(node1, invalid_node);
    }

    #[test]
    #[should_panic(expected = "connect failed")]
    fn test_connect_panic_self_loop() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);

        graph.connect(node, node);
    }

    #[test]
    #[should_panic(expected = "bfs failed")]
    fn test_bfs_panic_invalid() {
        let graph: Graph<i32> = Graph::new();
        let invalid_node = Key::null();

        graph.bfs(invalid_node);
    }

    #[test]
    #[should_panic(expected = "dfs failed")]
    fn test_dfs_panic_invalid() {
        let graph: Graph<i32> = Graph::new();
        let invalid_node = Key::null();

        graph.dfs(invalid_node);
    }

    #[test]
    #[should_panic(expected = "value failed")]
    fn test_value_panic_invalid() {
        let graph: Graph<i32> = Graph::new();
        let invalid_node = Key::null();

        graph.value(invalid_node);
    }

    // === Тесты итераторов ===

    #[test]
    fn test_iter_node() {
        let mut graph = Graph::new();
        let n0 = graph.new_node(0);
        let n1 = graph.new_node(1);
        let n2 = graph.new_node(2);

        let nodes: Vec<NodeIndex> = graph.iter_node().collect();
        assert_eq!(nodes.len(), 3);
        assert!(nodes.contains(&n0));
        assert!(nodes.contains(&n1));
        assert!(nodes.contains(&n2));
    }

    #[test]
    fn test_iter_value() {
        let mut graph = Graph::new();
        graph.new_node(10);
        graph.new_node(20);
        graph.new_node(30);

        let values: Vec<i32> = graph.iter_value().copied().collect();
        assert_eq!(values, vec![10, 20, 30]);
    }

    #[test]
    fn test_has_node() {
        let mut graph = Graph::new();
        let node = graph.new_node(42);
        let invalid = Key::null();

        assert!(graph.has_node(node));
        assert!(!graph.has_node(invalid));
    }

    #[test]
    fn test_free_list_reuse() {
        let mut graph = Graph::new();

        let _n0 = graph.new_node(0);
        let n1 = graph.new_node(1);
        let _n2 = graph.new_node(2);

        graph.try_del(n1).unwrap();

        let n3 = graph.new_node(3);
        // В slotmap ключи могут переиспользоваться, но поколение ключа отличается.
        // Поэтому старый n1 и новый n3 не равны.
        assert_eq!(graph.node_count(), 3);
        assert_eq!(*graph.try_value(n3).unwrap(), 3);
        assert_ne!(n3, n1);
    }

    #[test]
    fn test_complex_operations() {
        let mut graph = Graph::new();

        let nodes: Vec<NodeIndex> = (0..5).map(|i| graph.new_node(i * 10)).collect();

        graph.connect_all(nodes.iter().copied());

        for &node in &nodes {
            assert_eq!(graph.links(node).len(), nodes.len() - 1);
        }

        graph.disconnect(nodes[0], nodes[1]);
        graph.disconnect(nodes[0], nodes[2]);

        assert!(!graph.links(nodes[0]).contains(&nodes[1]));
        assert!(!graph.links(nodes[0]).contains(&nodes[2]));
        assert!(graph.links(nodes[0]).contains(&nodes[3]));
        assert!(graph.links(nodes[0]).contains(&nodes[4]));
    }

    #[test]
    fn test_find() {
        let mut graph = Graph::new();
        let n0 = graph.new_node(42);
        let _n1 = graph.new_node(100);
        let _n2 = graph.new_node(42);

        let found = graph.find(&42);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), n0); // первый найденный узел
        assert!(graph.find(&999).is_none());
    }

    #[test]
    fn test_are_connected() {
        let mut graph = Graph::new();
        let n0 = graph.new_node(0);
        let n1 = graph.new_node(1);
        let n2 = graph.new_node(2);

        graph.connect(n0, n1);

        assert!(graph.are_connected(n0, n1));
        assert!(graph.are_connected(n1, n0));
        assert!(!graph.are_connected(n0, n2));
    }
}