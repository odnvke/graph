mod simple_graph;
mod index_error_data;
mod ugraph;
mod wugraph;
mod actor;

use simple_graph::Graph;
use ugraph::UGraph;
use index_error_data::{NodeError, NodeIndex};

fn main() {
// ===== Тестирование UGraph (false, false) =====
println!("\n========== UGraph (false, false) ==========");

let mut ug = UGraph::new(false, false);  // без петель, без кратных рёбер
println!("1. Создан пустой ориентированный граф: {:?}", ug);

let u1 = ug.new_node(100);
let u2 = ug.new_node(200);
let u3 = ug.new_node(300);
println!("   Добавлены узлы: {:?}, {:?}, {:?}", u1, u2, u3);

// Добавление рёбер (направленные)
ug.connect(u1, u2);
ug.connect(u2, u3);
ug.connect(u3, u1);
println!("\n2. После добавления рёбер u1->u2, u2->u3, u3->u1:");
println!("   {:?}", ug);

// Проверка наличия рёбер
println!("\n3. Проверка связей:");
println!("   are_connected({:?}, {:?}) = {}", u1, u2, ug.are_connected(u1, u2));
println!("   are_connected({:?}, {:?}) = {}", u2, u1, ug.are_connected(u2, u1));
println!("   are_connected({:?}, {:?}) = {}", u1, u3, ug.are_connected(u1, u3));

// Соседи (исходящие и входящие)
println!("\n4. Соседи:");
println!("   neighbors_out({:?}) = {:?}", u1, ug.neighbors_out(u1));
println!("   neighbors_in({:?}) = {:?}", u1, ug.neighbors_in(u1));
println!("   iter_neighbors_out({:?}):", u1);
for nb in ug.iter_neighbors_out(u1) {
    println!("     -> {:?}", nb);
}

// Количество рёбер
println!("\n5. edge_count() = {}", ug.edge_count());  // должно быть 3

// Попытка добавить ребро, которое уже существует (псевдограф запрещён)
println!("\n6. Попытка добавить повторное ребро u1->u2 (должно проигнорироваться):");
ug.try_connect(u1, u2).unwrap();  // Ok, но ничего не добавит
println!("   edge_count() = {}", ug.edge_count());  // всё ещё 3

// Попытка создать петлю (self-loop) — запрещено
println!("\n7. Попытка добавить петлю u1->u1:");
match ug.try_connect(u1, u1) {
    Ok(()) => println!("   Успешно (не должно быть)"),
    Err(e) => println!("   Ожидаемая ошибка: {}", e),
}

// edge_count_between
println!("\n8. edge_count_between({:?}, {:?}) = {}", u1, u2, ug.edge_count_between(u1, u2));
println!("   edge_count_between({:?}, {:?}) = {}", u2, u1, ug.edge_count_between(u2, u1));

// Итераторы
println!("\n9. Итераторы:");
println!("   iter_node(): {:?}", ug.iter_node().collect::<Vec<_>>());
println!("   iter_value(): {:?}", ug.iter_value().collect::<Vec<_>>());
println!("   iter_edges(): {:?}", ug.iter_edges().collect::<Vec<_>>());

// Поиск по значению
println!("\n10. find(&200) = {:?}", ug.find(&200));

// BFS и DFS (по исходящим рёбрам)
println!("\n11. Обходы:");
println!("    BFS из {:?}: {:?}", u1, ug.bfs(u1));
println!("    DFS из {:?}: {:?}", u1, ug.dfs(u1));

// Замена значения
let old_val = ug.try_insert(u2, 250).unwrap();
println!("\n12. try_insert({:?}, 250) вернул {}, новое значение = {}", u2, old_val, ug[u2]);

// Удаление ребра
ug.disconnect(u2, u3);
println!("\n13. После disconnect({:?}, {:?}):", u2, u3);
println!("    are_connected({:?}, {:?}) = {}", u2, u3, ug.are_connected(u2, u3));
println!("    edge_count() = {}", ug.edge_count());
println!("    {:?}", ug);

// Удаление узла
ug.del(u2);
println!("\n14. После del({:?}):", u2);
println!("    has_node({:?}) = {}", u2, ug.has_node(u2));
println!("    node_count() = {}", ug.node_count());
println!("    {:?}", ug);

// connect_all (создание полного ориентированного графа между указанными узлами)
let mut ug2 = UGraph::new(false, false);
let a = ug2.new_node("A");
let b = ug2.new_node("B");
let c = ug2.new_node("C");
ug2.connect_all(vec![a, b, c]);  // добавляет рёбра в обе стороны между каждой парой
println!("\n15. connect_all (полный ориентированный граф на 3 вершинах):");
println!("    {:?}", ug2);

// loop_node — нет в UGraph, но есть try_connect_all, который делает полносвязный
// (метод loop_node есть только в Graph, не в UGraph)

// remove_all_edges — удаляет все рёбра между двумя узлами (в обе стороны)
let mut ug3 = UGraph::new(false, false);
let x = ug3.new_node(1);
let y = ug3.new_node(2);
ug3.connect(x, y);
ug3.connect(y, x);  // два направленных ребра
println!("\n16. До remove_all_edges: edge_count = {}", ug3.edge_count()); // 2
let removed = ug3.remove_all_edges(x, y);
println!("    remove_all_edges({:?}, {:?}) удалила {} рёбер", x, y, removed);
println!("    После: edge_count = {}", ug3.edge_count());

// Обработка ошибок
println!("\n17. Обработка ошибок:");
let fake = ug3.first_node().unwrap();
ug3.del(fake);  // удаляем узел
match ug3.try_connect(fake, y) {
    Ok(()) => println!("   Не должно быть"),
    Err(e) => println!("   try_connect с удалённым узлом: {}", e),
}

println!("\n✅ Все тесты UGraph(false, false) пройдены!");



// ===== Тестирование UGraph(true, false) =====
println!("\n========== UGraph(true, false) ==========");
println!("Параметры: self-loop = true, pseudo-graph = false");

let mut ug = UGraph::new(true, false);  // петли разрешены, кратные рёбра запрещены

let u1 = ug.new_node(10);
let u2 = ug.new_node(20);
let u3 = ug.new_node(30);
println!("Добавлены узлы: {:?}, {:?}, {:?}", u1, u2, u3);

// Добавляем рёбра, включая петлю
ug.connect(u1, u2);
ug.connect(u2, u3);
ug.connect(u3, u3);   // петля — разрешена!
println!("\n1. После добавления рёбер u1->u2, u2->u3, u3->u3 (петля):");
println!("   {:?}", ug);

// Проверка связей
println!("\n2. Проверка связей:");
println!("   are_connected({:?}, {:?}) = {}", u3, u3, ug.are_connected(u3, u3)); // true
println!("   are_connected({:?}, {:?}) = {}", u1, u3, ug.are_connected(u1, u3)); // false

// Соседи (исходящие и входящие)
println!("\n3. Соседи:");
println!("   neighbors_out({:?}) = {:?}", u3, ug.neighbors_out(u3)); // должно включать u3
println!("   neighbors_in({:?}) = {:?}", u3, ug.neighbors_in(u3));   // должно включать u3

// Попытка добавить кратное ребро (запрещено)
println!("\n4. Попытка добавить повторное ребро u1->u2 (должно проигнорироваться):");
ug.try_connect(u1, u2).unwrap();
println!("   edge_count() = {}", ug.edge_count()); // остаётся 3 (u1->u2, u2->u3, u3->u3)

// Попытка добавить ещё одну петлю на u3 (тоже проигнорируется)
println!("   Попытка добавить ещё одну петлю u3->u3:");
ug.try_connect(u3, u3).unwrap();
println!("   edge_count() = {}", ug.edge_count()); // всё ещё 3

// edge_count_between
println!("\n5. edge_count_between({:?}, {:?}) = {}", u3, u3, ug.edge_count_between(u3, u3)); // 1

// Обходы (петля не вызывает зацикливания, т.к. visited проверяется)
println!("\n6. BFS из {:?}: {:?}", u3, ug.bfs(u3));   // [u3, ...]
println!("   DFS из {:?}: {:?}", u3, ug.dfs(u3));

// Удаление петли через disconnect
ug.disconnect(u3, u3);
println!("\n7. После disconnect({:?}, {:?}) — петля удалена:", u3, u3);
println!("   are_connected({:?}, {:?}) = {}", u3, u3, ug.are_connected(u3, u3));
println!("   edge_count() = {}", ug.edge_count()); // 2

// connect_all — создаёт полный ориентированный граф между узлами (без петель)
let mut ug2 = UGraph::new(true, false);
let a = ug2.new_node('a');
let b = ug2.new_node('b');
let c = ug2.new_node('c');
ug2.connect_all(vec![a, b, c]);
println!("\n8. connect_all (полный орграф без петель):");
println!("   {:?}", ug2);

// Проверим, что петли не создаются connect_all
println!("   Есть ли петля на {:?}? {}", a, ug2.are_connected(a, a)); // false

// Удаление всех рёбер между двумя узлами (в обе стороны)
let mut ug3 = UGraph::new(true, false);
let x = ug3.new_node(1);
let y = ug3.new_node(2);
ug3.connect(x, y);
ug3.connect(y, x);
println!("\n9. remove_all_edges между {:?} и {:?}:", x, y);
println!("   До: edge_count = {}", ug3.edge_count()); // 2
let removed = ug3.remove_all_edges(x, y);
println!("   Удалено {} рёбер", removed);
println!("   После: edge_count = {}", ug3.edge_count()); // 0

// Проверка, что после удаления узла петли корректно удаляются
let mut ug4 = UGraph::new(true, false);
let p = ug4.new_node(42);
ug4.connect(p, p); // петля
ug4.del(p);
println!("\n10. Удаление узла с петлёй:");
println!("    has_node({:?}) = {}", p, ug4.has_node(p)); // false
println!("    node_count() = {}", ug4.node_count()); // 0

println!("\n✅ Все тесты UGraph(true, false) пройдены!");



// ===== Тестирование UGraph(false, true) =====
println!("\n========== UGraph(false, true) ==========");
println!("Параметры: self-loop = false, pseudo-graph = true (кратные рёбра разрешены)");

let mut ug = UGraph::new(false, true);

let u1 = ug.new_node(1);
let u2 = ug.new_node(2);
let u3 = ug.new_node(3);
println!("Добавлены узлы: {:?}, {:?}, {:?}", u1, u2, u3);

// Добавляем кратные рёбра
ug.connect(u1, u2);
ug.connect(u1, u2);  // второе ребро u1->u2
ug.connect(u2, u3);
ug.connect(u2, u3);  // второе ребро u2->u3
println!("\n1. После добавления кратных рёбер:");
println!("   {:?}", ug);

println!("2. edge_count() = {}", ug.edge_count());  // 4
println!("   edge_count_between({:?}, {:?}) = {}", u1, u2, ug.edge_count_between(u1, u2)); // 2
println!("   edge_count_between({:?}, {:?}) = {}", u2, u3, ug.edge_count_between(u2, u3)); // 2

// Попытка петли (запрещена)
println!("\n3. Попытка создать петлю u1->u1:");
match ug.try_connect(u1, u1) {
    Ok(()) => println!("   Успешно (не должно быть)"),
    Err(e) => println!("   Ожидаемая ошибка: {}", e),
}

// neighbours_out и neighbours_in с повторениями
println!("\n4. neighbors_out({:?}) = {:?}", u1, ug.neighbors_out(u1)); // [2, 2]
println!("   neighbors_in({:?}) = {:?}", u2, ug.neighbors_in(u2));     // [1, 1]

// Особенность: disconnect удаляет ВСЕ рёбра между парой (из-за retain)
println!("\n5. disconnect({:?}, {:?}) удаляет ВСЕ кратные рёбра (а не одно):", u1, u2);
ug.disconnect(u1, u2);
println!("   edge_count_between({:?}, {:?}) = {}", u1, u2, ug.edge_count_between(u1, u2)); // 0
println!("   edge_count() = {}", ug.edge_count()); // 2 (только два ребра u2->u3)

// Удаление всех рёбер между u2 и u3 (два ребра)
let removed = ug.remove_all_edges(u2, u3);
println!("\n6. remove_all_edges({:?}, {:?}) удалила {} рёбер", u2, u3, removed); // 2
println!("   edge_count_between = {}", ug.edge_count_between(u2, u3)); // 0
println!("   edge_count() = {}", ug.edge_count()); // 0

// connect_all с кратными рёбрами
let mut ug2 = UGraph::new(false, true);
let a = ug2.new_node('A');
let b = ug2.new_node('B');
let c = ug2.new_node('C');
ug2.connect_all(vec![a, b, c]);
println!("\n7. connect_all (первый вызов): edge_count = {}", ug2.edge_count()); // 6
ug2.connect_all(vec![a, b, c]);
println!("   После повторного connect_all: edge_count = {}", ug2.edge_count()); // 12
println!("   edge_count_between({:?}, {:?}) = {}", a, b, ug2.edge_count_between(a, b)); // 2

// Удаление узла (должно удалить все его рёбра)
let mut ug3 = UGraph::new(false, true);
let x = ug3.new_node(10);
let y = ug3.new_node(20);
ug3.connect(x, y);
ug3.connect(x, y); // два ребра
ug3.del(x);
println!("\n8. Удаление узла с кратными рёбрами:");
println!("   has_node({:?}) = {}", x, ug3.has_node(x));
println!("   edge_count() = {}", ug3.edge_count()); // 0

println!("\n✅ Все тесты UGraph(false, true) пройдены!");



// ===== Тестирование UGraph(true, true) =====
println!("\n========== UGraph(true, true) ==========");
println!("Параметры: self-loop = true, pseudo-graph = true (петли и кратные рёбра разрешены)");

let mut ug = UGraph::new(true, true);

let u1 = ug.new_node(100);
let u2 = ug.new_node(200);
let u3 = ug.new_node(300);
println!("Добавлены узлы: {:?}, {:?}, {:?}", u1, u2, u3);

// Кратные рёбра и петли
ug.connect(u1, u2);
ug.connect(u1, u2);      // второе ребро u1->u2
ug.connect(u2, u3);
ug.connect(u3, u3);      // петля
ug.connect(u3, u3);      // вторая петля на u3
println!("\n1. После добавления рёбер:\n   {:?}", ug);

println!("2. edge_count() = {}", ug.edge_count()); // 5
println!("   edge_count_between({:?}, {:?}) = {}", u1, u2, ug.edge_count_between(u1, u2)); // 2
println!("   edge_count_between({:?}, {:?}) = {}", u3, u3, ug.edge_count_between(u3, u3)); // 2

// neighbours_out и neighbours_in с повторениями
println!("\n3. neighbors_out({:?}) = {:?}", u1, ug.neighbors_out(u1)); // [2, 2]
println!("   neighbors_in({:?}) = {:?}", u2, ug.neighbors_in(u2));     // [1, 1]
println!("   neighbors_out({:?}) = {:?}", u3, ug.neighbors_out(u3));   // [3, 3]

// Особенность: disconnect удаляет ВСЕ рёбра между парой (из-за retain)
println!("\n4. disconnect({:?}, {:?}) удаляет ВСЕ кратные рёбра (оба):", u1, u2);
ug.disconnect(u1, u2);
println!("   edge_count_between({:?}, {:?}) = {}", u1, u2, ug.edge_count_between(u1, u2)); // 0
println!("   edge_count() = {}", ug.edge_count()); // 3 (u2->u3, u3->u3, u3->u3)

// disconnect на петлях также удаляет все петли
println!("   disconnect({:?}, {:?}) удаляет ВСЕ петли:", u3, u3);
ug.disconnect(u3, u3);
println!("   edge_count_between({:?}, {:?}) = {}", u3, u3, ug.edge_count_between(u3, u3)); // 0
println!("   edge_count() = {}", ug.edge_count()); // 1 (только u2->u3)

// Удаление всех рёбер между u2 и u3 (оставшееся одно ребро)
let removed = ug.remove_all_edges(u2, u3);
println!("\n5. remove_all_edges({:?}, {:?}) удалила {} ребро", u2, u3, removed); // 1
println!("   edge_count_between = {}", ug.edge_count_between(u2, u3)); // 0
println!("   edge_count() = {}", ug.edge_count()); // 0

// connect_all с кратными рёбрами
let mut ug2 = UGraph::new(true, true);
let a = ug2.new_node('A');
let b = ug2.new_node('B');
let c = ug2.new_node('C');
ug2.connect_all(vec![a, b, c]);
println!("\n6. connect_all (первый вызов): edge_count = {}", ug2.edge_count()); // 6
ug2.connect_all(vec![a, b, c]);
println!("   После повторного connect_all: edge_count = {}", ug2.edge_count()); // 12
println!("   edge_count_between({:?}, {:?}) = {}", a, b, ug2.edge_count_between(a, b)); // 2

// Добавляем петли вручную
ug2.connect(a, a);
ug2.connect(b, b);
ug2.connect(c, c);
println!("\n7. После добавления петель: edge_count = {}", ug2.edge_count()); // 15

// Удаляем все рёбра между разными узлами, оставляя петли
ug2.remove_all_edges(a, b);
ug2.remove_all_edges(a, c);
ug2.remove_all_edges(b, c);
println!("   После remove_all_edges между разными узлами: edge_count = {}", ug2.edge_count()); // 3 петли

println!("\n✅ Все тесты UGraph(true, true) пройдены!");
}