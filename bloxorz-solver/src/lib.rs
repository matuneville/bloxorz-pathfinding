mod graphs;
use crate::graphs::{bfs, add_adj};

pub fn level_solution(map : Vec<Vec<isize>>) -> isize {
    let mut map_nodes : Vec<Vec<isize>> = vec![vec![-1; map[0].len()]; map.len()]; // -1 where there are no cells
    let mut visited : Vec<Vec<Vec<bool>>> = vec![vec![vec![false,false,false]; map[0].len()]; map.len()];
    let mut node = 0;
    let mut src: (usize, usize) = (0,0);
    let mut dst: (usize, usize) = (0,0);

    for i in 0..map.len() {
        for j in 0..(map[0]).len() {
            if map[i][j] != 0 { // if that cell exists
                if map[i][j] == 2 { src = (i, j); }
                if map[i][j] == 3 { dst = (i, j); }
                map_nodes[i][j] = node;
                node += 1;
            }
        }
    }
    // now nodes is the total amount of "existing" cells in the game map
    let mut adj_list : Vec<Vec<isize>> = vec![vec![]; (3*node) as usize];

    add_adj(& map_nodes, &mut adj_list, &mut visited, src, map_nodes[src.0][src.1] as usize);

    /*for i in 0..adj_list.len(){
        print!("{} {}", i, ": ");
        for j in 0..adj_list[i].len() {
            print!("{} {}", adj_list[i][j], " ");
        }
        println!();
    }*/

    let root = map_nodes[src.0][src.1];
    let end: usize = map_nodes[dst.0][dst.1] as usize;
    let distances = bfs(&adj_list, root);

    return distances[end];
}