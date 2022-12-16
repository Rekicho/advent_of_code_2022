// Dijkstra implementation stolen from:
// https://doc.rust-lang.org/std/collections/binary_heap/index.html

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn get_index(row: usize, col: usize, row_len: usize) -> usize {
    (row * row_len) + col
}

fn get_possible_neighbours(
    row: usize,
    col: usize,
    last_col: usize,
    last_row: usize,
) -> Vec<(usize, usize)> {
    match (row, col) {
        // 4 Corners
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, col) if col == last_col => vec![(0, last_col - 1), (1, last_col)],
        (row, 0) if row == last_row => vec![(last_row - 1, 0), (last_row, 1)],
        (row, col) if row == last_row && col == last_col => {
            vec![(last_row - 1, last_col), (last_row, last_col - 1)]
        }

        // 4 Matrix "edges"
        (0, col) => vec![(0, col - 1), (1, col), (0, col + 1)],
        (row, 0) => vec![(row - 1, 0), (row, 1), (row + 1, 0)],
        (row, col) if col == last_col => {
            vec![
                (row - 1, last_col),
                (row, last_col - 1),
                (row + 1, last_col),
            ]
        }
        (row, col) if row == last_row => {
            vec![
                (last_row - 1, col),
                (last_row, col - 1),
                (last_row, col + 1),
            ]
        }

        // Rest
        (row, col) => vec![
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col),
        ],
    }
}

pub fn solve_a(input: String) -> Result<String, String> {
    let matrix: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut graph: Vec<Vec<Edge>> = Vec::new();
    let mut start_index = 0;
    let mut end_index = 0;

    let row_len = matrix[0].len();
    let num_rows = matrix.len();

    for (i, row) in matrix.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let mut current_node: Vec<Edge> = Vec::new();
            let index = get_index(i, j, row_len);
            let mut node_value = *node;

            // Check if start or end
            if node_value == 'S' {
                start_index = index;
                node_value = 'a';
            }
            if *node == 'E' {
                end_index = index;
                node_value = 'z';
            }

            let possible_neighbours = get_possible_neighbours(i, j, row_len - 1, num_rows - 1);

            for possible_neighbour in possible_neighbours {
                let neighbour_value = match matrix[possible_neighbour.0][possible_neighbour.1] {
                    'S' => 'a',
                    'E' => 'z',
                    value => value,
                };

                if neighbour_value as i32 - node_value as i32 <= 1 {
                    current_node.push(Edge {
                        node: get_index(possible_neighbour.0, possible_neighbour.1, row_len),
                        cost: 1,
                    });
                }
            }

            graph.push(current_node);
        }
    }

    let res = shortest_path(&graph, start_index, end_index).unwrap();

    Ok(res.to_string())
}

pub fn solve_b(input: String) -> Result<String, String> {
    let matrix: Vec<Vec<char>> = input.split('\n').map(|x| x.chars().collect()).collect();
    let mut graph: Vec<Vec<Edge>> = Vec::new();
    let mut end_index = 0;

    let row_len = matrix[0].len();
    let num_rows = matrix.len();

    // The most efficient way to solve the problem is to "correctly" and "fully" implement Dijkstra
    // i.e. get the shortest path between all nodes (and not just from start to end)
    // and then use it to find the closest possible start
    // I didn't bother to do that. Instead, I ran the "simplified" Dijkstra for all possible starts
    // and obtain the smallest one
    let mut possible_starts: Vec<usize> = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let mut current_node: Vec<Edge> = Vec::new();
            let index = get_index(i, j, row_len);
            let mut node_value = *node;

            // Check if start or end
            if node_value == 'S' {
                node_value = 'a';
            }
            if *node == 'E' {
                end_index = index;
                node_value = 'z';
            }

            // If node is at bottom - it may be a possible start
            if node_value == 'a' {
                possible_starts.push(index);
            }

            let possible_neighbours = get_possible_neighbours(i, j, row_len - 1, num_rows - 1);

            for possible_neighbour in possible_neighbours {
                let neighbour_value = match matrix[possible_neighbour.0][possible_neighbour.1] {
                    'S' => 'a',
                    'E' => 'z',
                    value => value,
                };

                if neighbour_value as i32 - node_value as i32 <= 1 {
                    current_node.push(Edge {
                        node: get_index(possible_neighbour.0, possible_neighbour.1, row_len),
                        cost: 1,
                    });
                }
            }

            graph.push(current_node);
        }
    }

    let res = possible_starts
        .iter()
        .map(|x| shortest_path(&graph, *x, end_index).unwrap_or(usize::MAX))
        .min()
        .unwrap();

    Ok(res.to_string())
}
