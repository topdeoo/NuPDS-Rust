use graph::{
    self,
    prelude::{GraphBuilder, UndirectedCsrGraph},
};

fn main() {
    let graph: UndirectedCsrGraph<usize> = GraphBuilder::new()
        .csr_layout(graph::prelude::CsrLayout::Unsorted)
        .edges(vec![(0, 1), (1, 2), (0, 2)])
        .build();

    println!("Hello, world!");
}
