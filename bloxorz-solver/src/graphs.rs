use std::collections::VecDeque;


// position 0 : standing
// position 1 : horizontal in X axis
// position 2 : horizontal in Y axis

pub fn next_move(map_nodes: &Vec<Vec<isize>>, adj_list: &mut Vec<Vec<isize>>, visited: &mut Vec<Vec<Vec<bool>>>, (x,y): (usize, usize), node: usize,
                 offset_x: isize, offset_y: isize, position: isize){
    let ox = x as isize + offset_x;
    let oy = y as isize + offset_y;

    if ox < 0 || ox >= map_nodes.len() as isize ||  oy < 0 || oy >= map_nodes[0].len() as isize { return; }

    let ox = ox as usize;
    let oy = oy as usize;

    let range: isize = (adj_list.len() / 3) as isize;
    let next_node: isize = map_nodes[ox][oy];

    if next_node != -1 {
        if position == 1 {
            if (oy) as isize - 1 < 0 { return; }
            else if map_nodes[ox][oy - 1] == -1 { return; }
        }
        if position == 2 {
            if (ox) as isize + 1 >= map_nodes.len() as isize { return; }
            else if map_nodes[ox+1][oy] == -1 { return; }
        }

        adj_list[node].push(map_nodes[ox][oy]+ position*range);
        if !(visited[ox][oy][position as usize]) {
            add_adj(map_nodes, adj_list, visited, (ox, oy), (next_node + position * range) as usize);
        }
    }
}

pub fn add_adj(map_nodes: &Vec<Vec<isize>>, adj_list: &mut Vec<Vec<isize>>, visited: &mut Vec<Vec<Vec<bool>>>, (x,y): (usize, usize), node: usize){
    let range: usize = (adj_list.len())/ 3;

    // if the player is standing
    if node < range {
        visited[x][y][0] = true; // check as visited
        // move north
        next_move(map_nodes, adj_list, visited, (x,y), node, -2, 0, 2);
        // move west
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, -1, 1);
        // move south
        next_move(map_nodes, adj_list, visited, (x,y), node, 1, 0, 2);
        // move east
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, 2, 1);
    }
    // if the player is horizontal in the X axis
    else if node < 2*range{
        visited[x][y][1] = true; // check as visited
        // move north
        next_move(map_nodes, adj_list, visited, (x,y), node, -1, 0, 1);
        // move west
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, -2, 0);
        // move south
        next_move(map_nodes, adj_list, visited, (x,y), node, 1, 0, 1);
        // move east
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, 1, 0);
    }
    // if the player is horizontal in the Y axis
    else {
        visited[x][y][2] = true; // check as visited
        // move north
        next_move(map_nodes, adj_list, visited, (x,y), node, -1, 0, 0);
        // move west
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, -1, 2);
        // move south
        next_move(map_nodes, adj_list, visited, (x,y), node, 2, 0, 0);
        // move east
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, 1, 2);
    }
}

pub fn bfs(adj_list: &Vec<Vec<isize>>, root: isize) -> Vec<isize> {
    let mut visited = vec![false; adj_list.len()];
    let mut distances = vec![-1; adj_list.len()]; // Initialize distances with -1
    let mut queue = VecDeque::new();

    // Start from the root node
    visited[root as usize] = true;
    distances[root as usize] = 0; // Distance to root is 0
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        // Go through all the neighbors of the current node
        for &neighbor in &adj_list[node as usize] {
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                // The distance to the neighbor is the distance to the current node plus one
                distances[neighbor as usize] = distances[node as usize] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    distances
}