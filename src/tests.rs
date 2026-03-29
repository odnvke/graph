




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