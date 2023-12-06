use crate::bloxorz_path::create_graph;

mod bloxorz_path;

fn main() {
    println!("Hello, world!");
    let map: Vec<Vec<isize>> = vec![vec![1, 1, 1, 1],
                                    vec![1, 1, 1, 1],
                                    vec![1, 1, 1, 1]];
    create_graph(map, (0,0), (1,2));
}
