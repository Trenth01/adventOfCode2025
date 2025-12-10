use itertools::Itertools;
use petgraph::algo::min_spanning_tree;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use petgraph::visit::Dfs;
use std::collections::{HashMap, HashSet};


pub fn part1(input: &str) {
    let mut graph = Graph::<(i64,i64,i64), f64, Undirected>::new_undirected();
    let num_connections = 1000;
    let coords: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|n| n.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    let node_indices: Vec<NodeIndex> = coords.iter()
        .map(|&coord| graph.add_node(coord))
        .collect();

    let edges: Vec<((i64, i64, i64), (i64, i64, i64))> = coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b))   // dereference &tuple → tuple
        .collect();
    let mut distances: Vec<((i64, i64, i64), (i64, i64, i64), f64)> = edges
        .into_iter()
        .map(|(a, b)| (a, b, calculate_distance(a, b)))
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let mut shortest_edges = distances.clone();
    shortest_edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    let shortest_edges = shortest_edges.into_iter().take(num_connections);
    for (a_coord, b_coord, dist) in shortest_edges {
        let a_idx = node_indices.iter().find(|&&idx| graph[idx] == a_coord).unwrap();
        let b_idx = node_indices.iter().find(|&&idx| graph[idx] == b_coord).unwrap();
        graph.add_edge(*a_idx, *b_idx, dist);
    }
    let mut visited = HashSet::new();
    let mut sizes = Vec::new();

    for node in graph.node_indices() {
        if visited.contains(&node) { continue; }
        let mut dfs = Dfs::new(&graph, node);
        let mut count = 0;
        while let Some(nx) = dfs.next(&graph) {
            visited.insert(nx);
            count += 1;
        }
        sizes.push(count);
    }

    let mut size_counts = HashMap::new();
    for s in sizes {
        *size_counts.entry(s).or_insert(0) += 1;
    }
    let three_largest_sizes: Vec<usize> = {
        let mut s = size_counts.keys().cloned().collect::<Vec<usize>>();
        s.sort_unstable_by(|a, b| b.cmp(a));
        s.into_iter().take(3).collect()
    };

    println!("Part 1: {}", three_largest_sizes.iter().product::<usize>())
}

pub fn part2(input: &str) {
    let coords: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|l| {
            let mut it = l.split(',').map(|n| n.parse::<i64>().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect();

    let edges: Vec<((i64, i64, i64), (i64, i64, i64))> = coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (*a, *b))   // dereference &tuple → tuple
        .collect();
    let mut distances: Vec<((i64, i64, i64), (i64, i64, i64), f64)> = edges
        .into_iter()
        .map(|(a, b)| (a, b, calculate_distance(a, b)))
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Kruskal's algorithm: add edges in order of weight, skip if it creates a cycle
    let mut parent: HashMap<(i64, i64, i64), (i64, i64, i64)> = HashMap::new();
    for &coord in &coords {
        parent.insert(coord, coord);
    }

    let mut mst_edges: Vec<((i64, i64, i64), (i64, i64, i64), f64)> = Vec::new();
    let mut num_edges = 0;

   for (a, b, dist) in distances {
        let root_a = find_root(&mut parent, a);
        let root_b = find_root(&mut parent, b);

        if root_a != root_b {
            // Different components, add edge to MST
            parent.insert(root_a, root_b);
            mst_edges.push((a, b, dist));
            num_edges += 1;

            // MST has exactly n-1 edges for n nodes
            if num_edges == coords.len() - 1 {
                break;
            }
        }
    }
    let longest_edge = mst_edges.iter().max_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap());
    if let Some((a, b, dist)) = longest_edge {
        // println!("Part 2 (longest edge): {:?} -- {:?}, distance: {}", a, b, dist);
        println!("Part 2: {}", (a.0 * b.0));
    }

}

pub fn solve(input: &str) {
    part1(input);
    part2(input);
}

fn calculate_distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> f64{
    return (((a.0 - b.0).abs().pow(2) + (a.1 - b.1).abs().pow(2) + (a.2 - b.2).abs().pow(2)) as f64).sqrt();
}
fn find_root(parent: &mut HashMap<(i64, i64, i64), (i64, i64, i64)>, mut node: (i64, i64, i64)) -> (i64, i64, i64) {
    while parent[&node] != node {
        node = parent[&node];
    }
    node
}
