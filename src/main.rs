use std::collections::VecDeque;

fn main() {
    let mut n1 = Graph::new(0, 10);
    let mut n2 = Graph::new(1, 20); 
    
    n2.connect(&mut n1);

    n1.print_all_node();

    n2.disconnect(&mut n1);

    n1.print_all_node();
}



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