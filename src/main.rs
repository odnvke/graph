use std::collections::VecDeque;

fn main() {
    let mut n1 = Graph::new(0, 10);
    let mut n2 = Graph::new(1, 20);
    let mut n3 = Graph::new(2, 10);
    let mut n4 = Graph::new(3, 20);

    let mut n5 = Graph::new(4, 20);


    n1.connect(&mut n2);
    n2.connect(&mut n3);
    n3.connect(&mut n4);
    n4.connect(&mut n1);

    n5.connect(&mut n1);
    n5.connect(&mut n2);

    println!();

    n3.print_all_node();
    println!();
    
    n2.print_all_node();
    println!();

    n5.print_all_node();;
}

// не пиши просто код, цель проэкта это обуцение, обьясняй что и почему, не биси бональностями и пустыми словами, 
// не пиши код лучше пиши наводяшее или как начать что то писать
// НЕ БИСИ ПУСТМИ СЛОВАМИ

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

        println!("node-{} <-> node-{}", self.idx, gn.idx());
    }

    pub fn idx(&self) -> usize {
        self.idx
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
        let mut all_node: Vec<&mut Graph<T>> = Vec::with_capacity(64);

        //let self_ptr = self as *mut _;

        check_vec.push_front(self);

        loop {
            let Some(process_graph) = check_vec.pop_back() else { break };
            
            
            println!("  node-{}", (process_graph).idx());

            process_graph.visited = true;
            
            for g in process_graph.neighbors_mut() {
                if !g.visited {
                    g.visited = true;
                    check_vec.push_front(g);
                }
            };
        }
    }
}