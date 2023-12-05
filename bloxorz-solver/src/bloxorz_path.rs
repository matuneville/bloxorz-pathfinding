// position 0 : standing
// position 1 : horizontal in X axis
// position 2 : horizontal in Y axis

pub fn next_move(map_nodes: &Vec<Vec<isize>>, adj_list: &mut Vec<Vec<isize>>, visited: &mut Vec<Vec<(bool, bool, bool)>>, (x,y): (usize, usize), node: usize,
                 offset_x: isize, offset_y: isize, position: isize){

    let range: isize = (map_nodes.len() / 3) as isize;
    let next_node: isize = map_nodes[x + offset_x][y + offset_y];

    // TODO : check both using cells when moving to horizontal !!!
    // if (position == 1)

    if next_node != -1 && !(visited[x+offset_x][y+offset_y][position]){
        adj_list[node].push(map_nodes[x+offset_x][y+offset_y]+ position*range);
        add_adj(map_nodes, adj_list, visited, (x+offset_x, y+offset_y), (next_node+position*range) as usize);
    }
}

pub fn add_adj(map_nodes: &Vec<Vec<isize>>, adj_list: &mut Vec<Vec<isize>>, visited: &mut Vec<Vec<(bool, bool, bool)>>, (x,y): (usize, usize), node: usize){
    // if the player is standing
    let range: usize = map_nodes.len()/ 3;
    if 0 <= node && node <= range {
        visited[x][y] = (true, visited[x][y][1], visited[x][y][2]); // check as visited
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
    else if range < node && node <= 2*range{
        visited[x][y] = (visited[x][y][0], true, visited[x][y][2]); // check as visited
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
        visited[x][y] = (visited[x][y][0], visited[x][y][1], true); // check as visited
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
pub fn create_graph(map : Vec<Vec<isize>>, src : (usize, usize), dst : (usize, usize)) -> Vec<Vec<isize>>{
    let mut map_nodes : Vec<Vec<isize>> = vec![vec![map[0].len() as isize, -1], map.len()]; // -1 where there are no cells
    let mut node = 0;
    for i in map.len() {
        for j in (map[0]).len() {
            if map[i][j] == 1 { // if that cell exists
                map_nodes[i][j] = node;
                node += 1;
            }
        }
    }
    // now nodes is the total amount of "existing" cells in the game map
    let adj_list : Vec<Vec<isize>> = vec![vec![], 3*(node+1)];


    return;
}