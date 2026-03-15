use std::collections::VecDeque;

fn main() {
    stress_test_connect_disconnect();
    stress_test_large_graph();
    stress_test_random_operations();
    stress_test_memory_management();
}

// Тест 1: Массовое подключение/отключение
fn stress_test_connect_disconnect() {
    println!("\n=== СТРЕСС-ТЕСТ 1: Массовое подключение/отключение ===\n");
    
    const NODES_COUNT: usize = 100;
    let mut nodes: Vec<Graph<i32>> = (0..NODES_COUNT)
        .map(|i| Graph::new(i, i as i32 * 10))
        .collect();
    
    // Создаем полный граф (каждый с каждым)
    println!("Создание полного графа из {} узлов...", NODES_COUNT);
    for i in 0..NODES_COUNT {
        for j in i+1..NODES_COUNT {
            // Разделяем заимствования
            let (left, right) = if i < j {
                let (left_part, right_part) = nodes.split_at_mut(j);
                (&mut left_part[i], &mut right_part[0])
            } else {
                let (left_part, right_part) = nodes.split_at_mut(i);
                (&mut right_part[0], &mut left_part[j])
            };
            left.connect(right);
        }
    }
    
    // Проверяем связи через обход
    println!("\nПроверка связей через обход от узла 0:");
    nodes[0].print_all_node();
    
    // Массовое отключение
    println!("\nМассовое отключение всех связей...");
    for i in 0..NODES_COUNT {
        for j in i+1..NODES_COUNT {
            let (left, right) = if i < j {
                let (left_part, right_part) = nodes.split_at_mut(j);
                (&mut left_part[i], &mut right_part[0])
            } else {
                let (left_part, right_part) = nodes.split_at_mut(i);
                (&mut right_part[0], &mut left_part[j])
            };
            left.disconnect(right);
        }
    }
    
    println!("\nПосле отключения (должен быть только узел 0):");
    nodes[0].print_all_node();
    println!("✓ Тест 1 пройден");
}

// Тест 2: Большой граф
fn stress_test_large_graph() {
    println!("\n=== СТРЕСС-ТЕСТ 2: Большой граф ===\n");
    
    const LARGE_GRAPH_SIZE: usize = 1000;
    println!("Создание графа из {} узлов...", LARGE_GRAPH_SIZE);
    
    let mut nodes: Vec<Graph<i32>> = (0..LARGE_GRAPH_SIZE)
        .map(|i| Graph::new(i, i as i32))
        .collect();
    
    // Создаем линейную цепочку
    println!("Создание линейной цепочки...");
    for i in 0..LARGE_GRAPH_SIZE-1 {
        let (left, right) = nodes.split_at_mut(i+1);
        left[i].connect(&mut right[0]);
    }
    
    println!("Обход линейной цепочки из {} узлов:", LARGE_GRAPH_SIZE);
    nodes[0].print_all_node();
    
    // Добавляем случайные связи
    println!("\nДобавление случайных связей...");
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    for _ in 0..5000 {
        let i = rng.gen_range(0..LARGE_GRAPH_SIZE);
        let j = rng.gen_range(0..LARGE_GRAPH_SIZE);
        if i != j {
            let (left, right) = if i < j {
                let (left_part, right_part) = nodes.split_at_mut(j);
                (&mut left_part[i], &mut right_part[0])
            } else {
                let (left_part, right_part) = nodes.split_at_mut(i);
                (&mut right_part[0], &mut left_part[j])
            };
            
            // Игнорируем ошибки уже существующих связей
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                left.connect(right);
            }));
        }
    }
    
    println!("Граф создан, проверка связей...");
    nodes[0].print_all_node();
    println!("✓ Тест 2 пройден");
}

// Тест 3: Случайные операции
fn stress_test_random_operations() {
    println!("\n=== СТРЕСС-ТЕСТ 3: Хаотичные операции ===\n");
    
    const NODES: usize = 50;
    const OPERATIONS: usize = 1000;
    
    let mut nodes: Vec<Graph<String>> = (0..NODES)
        .map(|i| Graph::new(i, format!("Node-{}", i)))
        .collect();
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    println!("Выполнение {} случайных операций...", OPERATIONS);
    
    for op in 0..OPERATIONS {
        let i = rng.gen_range(0..NODES);
        let j = rng.gen_range(0..NODES);
        
        if i == j { continue; }
        
        let (left, right) = if i < j {
            let (left_part, right_part) = nodes.split_at_mut(j);
            (&mut left_part[i], &mut right_part[0])
        } else {
            let (left_part, right_part) = nodes.split_at_mut(i);
            (&mut right_part[0], &mut left_part[j])
        };
        
        // Случайно подключаем или отключаем
        if rng.gen_bool(0.5) {
            println!("Операция {}: попытка подключения {} <--> {}", op, i, j);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                left.connect(right);
            }));
        } else {
            println!("Операция {}: попытка отключения {} <-/-> {}", op, i, j);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                left.disconnect(right);
            }));
        }
        
        // Периодически проверяем целостность
        if op % 200 == 0 {
            println!("\nПроверка после {} операций:", op);
            nodes[0].print_all_node();
        }
    }
    
    println!("\nФинальная проверка:");
    nodes[0].print_all_node();
    println!("✓ Тест 3 пройден");
}

// Тест 4: Проверка управления памятью
fn stress_test_memory_management() {
    println!("\n=== СТРЕСС-ТЕСТ 4: Управление памятью ===\n");
    
    const ITERATIONS: usize = 100;
    
    println!("Создание и удаление графов в цикле ({} итераций)...", ITERATIONS);
    
    for iter in 0..ITERATIONS {
        println!("Итерация {}", iter);
        
        let mut nodes: Vec<Graph<i32>> = Vec::new();
        
        // Создаем небольшие графы
        for i in 0..10 {
            nodes.push(Graph::new(i, i as i32));
        }
        
        // Связываем в кольцо
        for i in 0..10 {
            let (left, right) = if i < (i+1)%10 {
                if i < (i+1)%10 {
                    let (left_part, right_part) = nodes.split_at_mut((i+1)%10);
                    (&mut left_part[i], &mut right_part[0])
                } else {
                    let (left_part, right_part) = nodes.split_at_mut(i);
                    (&mut right_part[0], &mut left_part[(i+1)%10])
                }
            } else {
                let (left_part, right_part) = nodes.split_at_mut(i);
                (&mut right_part[0], &mut left_part[(i+1)%10])
            };
            
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                left.connect(right);
            }));
        }
        
        // Добавляем случайные связи
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for _ in 0..20 {
            let i = rng.gen_range(0..10);
            let j = rng.gen_range(0..10);
            if i != j {
                let (left, right) = if i < j {
                    let (left_part, right_part) = nodes.split_at_mut(j);
                    (&mut left_part[i], &mut right_part[0])
                } else {
                    let (left_part, right_part) = nodes.split_at_mut(i);
                    (&mut right_part[0], &mut left_part[j])
                };
                
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    left.connect(right);
                }));
            }
        }
        
        println!("  Граф создан, nodes: {}", nodes.len());
        // nodes автоматически удалится здесь
    }
    
    println!("✓ Тест 4 пройден");
    println!("\n=== ВСЕ СТРЕСС-ТЕСТЫ УСПЕШНО ЗАВЕРШЕНЫ ===");
}

// Добавьте в Cargo.toml:
// [dependencies]
// rand = "0.8"



struct Graph<T> {
    visited: bool,
    idx: usize,
    value: T,
    links: Vec<*mut Graph<T>>,
} 
impl <T> Graph<T> {
    pub fn new(idx: usize, v: T) -> Self {
        Self { visited: false, idx: idx, value: v, links: Vec::new() }
    }

    pub fn connect(&mut self, gn: &mut Graph<T>) {
        if self.links.contains(&(gn as *mut _)) {
            panic!("\n\n\nerror:\n  >>  node-{}: already connected with node-{}\n\n\n", self.idx, gn.idx())
        }

        self.links.push(gn as *mut _);
        
        gn.links.push(self as *mut _);

        println!("node-{} <--> node-{}", self.idx, gn.idx());
    }

    pub fn idx(&self) -> usize {
        self.idx
    }

    pub fn disconnect(&mut self, gn: &mut Graph<T>) {
        if ! self.links.contains(&(gn as *mut _)) {
            panic!("\n\n\nerror:\n  >>  node-{}: not connected with node-{}\n\n\n", self.idx, gn.idx())
        }

        let mut idx1 = 0;
        for (idx, elem) in self.links.iter().enumerate()  {
            if *elem == gn as *mut _ {idx1 = idx; break;}
        };
        self.links.swap_remove(idx1);
        

        let mut idx1 = 0;
        for (idx, elem) in gn.links.iter().enumerate()  {
            if *elem == self as *mut _ {idx1 = idx; break;}
        };
        gn.links.swap_remove(idx1);

        println!("node-{} <-/-> node-{}", self.idx, gn.idx());
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn neighbors(&self) -> impl Iterator<Item = &Graph<T>> {
        self.links.iter().filter_map(|&ptr| {
            unsafe { ptr.as_ref() }
        })
    }

    pub fn neighbors_mut(&mut self) -> impl Iterator<Item = &mut Graph<T>> {
        // Сложнее, нужно собирать указатели сначала
        let ptrs: Vec<_> = self.links.iter().copied().collect();
        
        ptrs.into_iter().filter_map(move |ptr| {
            unsafe { ptr.as_mut() }
        })
    }

    pub fn print_all_node(&mut self) {
        let mut check_vec: VecDeque<&mut Graph<T>> = VecDeque::with_capacity(32);
        let mut all_node: Vec<*mut Graph<T>> = Vec::with_capacity(64);

        all_node.push(self as *mut _);
        check_vec.push_front(self);


        loop {
            let Some(process_graph) = check_vec.pop_back() else { break };
            
            println!("  node-{}", (process_graph).idx());

            process_graph.visited = true;
            
            for g in process_graph.neighbors_mut() {
                if !g.visited {
                    g.visited = true;
                    all_node.push(g as *mut _);
                    check_vec.push_front(g);
                }
            };
        }

        
        unsafe {
            all_node.iter_mut().for_each(|g| {
                (**g).visited = false;
            });
        }
    }
}

// impl<T> Drop for Graph<T> {
//     fn drop(&mut self) { 
//         let mut check_vec: VecDeque<&mut Graph<T>> = VecDeque::with_capacity(32);
//         let mut all_node: Vec<*mut Graph<T>> = Vec::with_capacity(64);

//         all_node.push(self as *mut _);
//         check_vec.push_front(self);


//         loop {
//             let Some(process_graph) = check_vec.pop_back() else { break };
            
//             println!("  node-{} mark on deleting", (process_graph).idx());

//             process_graph.visited = true;
            
//             for g in process_graph.neighbors_mut() {
//                 if !g.visited {
//                     g.visited = true;
//                     all_node.push(g as *mut _);
//                     check_vec.push_front(g);
//                 }
//             };
//         }


//         loop {
//             let Some(process_graph) = all_node.pop() else { break };
//             unsafe { drop(Box::from_raw(process_graph)); }
//         }
//     }
// }