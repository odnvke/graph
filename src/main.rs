mod simple_graph;
mod index_error_data;
mod ugraph;

use simple_graph::Graph;
use index_error_data::{NodeError, NodeIndex};

fn main() {
    // Создаём граф
    let mut g = Graph::new();

    // Создаём две ноды с произвольными значениями (0 и 1)
    let node_from = g.new_node(0); // "ниоткуда"
    let node_to   = g.new_node(1); // "никуда"

    // Соединяем их
    g.connect(node_from, node_to); // использует паникующую версию из panic_from_try!

    // Проверяем
    println!("Ноды соединены: {}", g.are_connected(node_from, node_to));
    println!("{:?}", g);
}