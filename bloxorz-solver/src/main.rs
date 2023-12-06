use crate::bloxorz_path::level_solution;

mod bloxorz_path;
mod graphs;

fn main() {
    println!("Hello, world!");
    let map: Vec<Vec<isize>> = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                                    vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 3],
                                    vec![1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1],
                                    vec![2, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0],
                                    vec![1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0],
                                    vec![0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0],
                                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0]];
    print!("{}", level_solution(map));
}
