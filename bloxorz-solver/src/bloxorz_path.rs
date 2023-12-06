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
        if position == 1 && map_nodes[ox][oy-1] == -1 { return; }
        if position == 2 && map_nodes[ox+1][oy] == -1 { return; }

        adj_list[node].push(map_nodes[ox][oy]+ position*range);
        if !(visited[ox][oy][position as usize]) {
            add_adj(map_nodes, adj_list, visited, (ox, oy), (next_node + position * range) as usize);
        }
    }
}

pub fn add_adj(map_nodes: &Vec<Vec<isize>>, adj_list: &mut Vec<Vec<isize>>, visited: &mut Vec<Vec<Vec<bool>>>, (x,y): (usize, usize), node: usize){
    // if the player is standing
    let range: usize = (adj_list.len())/ 3;
    if node < range {
        visited[x][y][0] = true; // check as visited
        // move north
        next_move(map_nodes, adj_list, visited, (x,y), node, -2, 0, 2);
        // move west
        next_move(map_nodes, adj_list, visited, (x,y), node, 0, -2, 1);
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


// returns adjacency list
pub fn create_graph(map : Vec<Vec<isize>>, src : (usize, usize), _dst : (usize, usize)) /*-> Vec<Vec<isize>>*/{
    let mut map_nodes : Vec<Vec<isize>> = vec![vec![-1; map[0].len()]; map.len()]; // -1 where there are no cells
    let mut visited : Vec<Vec<Vec<bool>>> = vec![vec![vec![false,false,false]; map[0].len()]; map.len()];
    let mut node = 0;
    for i in 0..map.len() {
        for j in 0..(map[0]).len() {
            if map[i][j] == 1 { // if that cell exists
                map_nodes[i][j] = node;
                node += 1;
            }
        }
    }
    // now nodes is the total amount of "existing" cells in the game map
    let mut adj_list : Vec<Vec<isize>> = vec![vec![]; (3*node) as usize];

    add_adj(& map_nodes, &mut adj_list, &mut visited, src, map_nodes[src.0][src.1] as usize);

    return;
}