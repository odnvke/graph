// benches/bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use graph::Graph;

fn bench_connect_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_operations");
    
    // Тестируем разные размеры
    for size in [50, 100, 200, 500].iter() {
        group.bench_function(format!("connect_all_{}", size), |b| {
            b.iter(|| {
                let mut g = Graph::new();
                let nodes: Vec<_> = (0..*size)
                    .map(|i| g.get_new_node(black_box(i)))
                    .collect();
                g.connect_all(nodes);
            })
        });
    }
    
    group.finish();
}
fn bench_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bfs");
    
    // for size in [50, 100, 200, 500].iter() {
    //     // Создаем граф один раз для каждого размера
    //     let mut g = Graph::new();
    //     let mut nodes = Vec::new();
    //     for i in 0..*size {
    //         nodes.push(g.get_new_node(i));
    //     }
    //     g.connect_all(nodes);
        
    //     group.bench_function(format!("bfs_{}", size), |b| {
    //         b.iter(|| {
    //             g.bfs(nodes[0])
    //         })
    //     });
    // }
    
    group.finish();
}

criterion_group!(benches, bench_bfs);
criterion_main!(benches);