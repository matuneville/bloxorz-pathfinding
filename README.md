# Bloxorz: finding solution to levels

!License: MIT

## Introduction

This repository contains an algorithmic solution to the Bloxorz game using graph modeling. The solution finds the shortest path to reach the goal.

## Approach

The approach to solving the Bloxorz game involves modeling the problem as a **three-level graph**, where each node represents a valid position that the prism can occupy. The three levels correspond to the three possible orientations of the prism: vertical, horizontal along the x-axis, and horizontal along the y-axis.

### Graph Modeling

To make an appropiate graph model, the map is needed as a matrix, which dimensions will be 
- Vertical: the length of the longest column of positions of the map
- Horizontal: the length of the longest row of positions of the map  

and its elements meaning are:
- `0` is an invalid position of the map (where there is no little square to stand the prism),
- `1` is a valid position (where there actually is a little square to stand the prism),
- and `2` and `3` the beginning and ending positions.  

Example:  

![image]()

Once the map is hardcoded as a matrix, the graph (with adjacency list representation) will be modeled with the following properties.  

Each **node** represents a valid state of the prism, defined by its position in the*matrix-represented map of the level, and its orientation. The **edges** between nodes represent valid movements of the prism from one state to another. The graph is divided into **three levels** corresponding to the three possible orientations of the prism:

1. **Vertical Orientation**: Which represents the prism standing upright.
2. **Horizontal X-Axis Orientation**: Which represents the prism lying flat along the x-axis, two positions on the matrix-represented map.
3. **Horizontal Y-Axis Orientation**: Same as before, but along the y-axis.

Each level of the graph contains nodes representing all valid positions that the prism can occupy in that particular orientation.

### Path Finding

Once the graph is constructed, the problem of finding the shortest path to the goal is solved using the **Breadth-First Search (BFS)** algorithm. In the context of this problem, the distance between two nodes (i.e., the weight of an edge) represents a single movement of the prism. Therefore, the shortest path corresponds to the minimum number of movements required to reach the goal.  

By applying BFS, we can efficiently find the shortest path in the graph, which corresponds to the optimal solution to the Bloxorz game.

## Usage

## License

