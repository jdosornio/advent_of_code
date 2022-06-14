use fancy_regex::Regex;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

struct DisjointSet {
    parent: Vec<usize>,
    height: Vec<u32>,
}

impl DisjointSet {
    fn new(nodes: usize) -> Self {
        DisjointSet {
            parent: (0..nodes).collect(),
            height: vec![1; nodes],
        }
    }

    fn parent(&mut self, element: usize) -> usize {
        let mut root = self.parent[element];

        while root != self.parent[root] {
            self.parent[root] = self.parent[self.parent[root]];
            root = self.parent[root];
        }

        root
    }

    fn are_connected(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.parent(a);
        let b_root = self.parent(b);

        a_root == b_root
    }

    fn connect(&mut self, a: usize, b: usize) {
        let a_root = self.parent(a);
        let b_root = self.parent(b);

        if a_root == b_root {
            return;
        }

        let a_height = self.height[a_root];
        let b_height = self.height[b_root];

        match a_height.cmp(&b_height) {
            Ordering::Less => {
                self.parent[a_root] = b_root;
            }
            Ordering::Equal => {
                self.parent[a_root] = b_root;
                self.height[b_root] += 1;
            }
            Ordering::Greater => {
                self.parent[b_root] = a_root;
            }
        }
    }
}

struct Edge<'a> {
    start: &'a str,
    end: &'a str,
    cost: u32,
}

// TODO: Create a struct to order edge elements

pub fn puzzle1(input: &str) -> u32 {
    let edge_re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut nodes = HashMap::<&str, usize>::new();
    let mut total_nodes: usize = 0;
    let edge_cost: HashMap<HashSet<usize>, u32> = HashMap::new();

    // Parse all the edges, get all the distinct cities
    input.lines().for_each(|line| {
        let captures = edge_re.captures(line).unwrap().unwrap();
        let start = captures.get(1).unwrap().as_str();
        let end = captures.get(2).unwrap().as_str();
        // let cost = captures.get(3).unwrap().as_str().parse().unwrap();

        if !nodes.contains_key(start) {
            nodes.insert(start, total_nodes);
            total_nodes += 1;
        }
        if !nodes.contains_key(end) {
            nodes.insert(end, total_nodes);
            total_nodes += 1;
        }

        // edge_cost.insert(vec![0].collect(), cost);
    });
    // // Create a DisjointSet to check for connection
    // let mut node_set = DisjointSet::new(total_nodes);
    // // Order edges by cost or insert all in min heap
    // edges.sort_by(|edge_1, edge_2| edge_1.cost.cmp(&edge_2.cost));
    // // For 1 to n - 1 edges, start acc the total cost
    // let mut edge_count = 0;
    // let mut current_idx = 0;

    // while edge_count < total_nodes - 1 {
    //     let current_edge = &edges[current_idx];
    //     let start = nodes[current_edge.start];
    //     let end = nodes[current_edge.end];
    //     let cost = current_edge.cost;

    //     if !node_set.are_connected(start, end) {
    //         node_set.connect(start, end);
    //         mst_cost += cost;
    //         edge_count += 1;
    //     }
    //     current_idx += 1;
    // }

    // mst_cost

    (0..8).permutations(8).for_each(|comb| {
        // Get cost of route and keep the lowest one
        println!("{:?}", comb);
        comb.iter().tuple_windows().for_each(|(a, b)| {
            print!("({}, {}) ", a, b);
        });
        println!();
    });

    0
}

// pub fn puzzle2(input: &str) -> u32 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle1_test() {
        let tests = [(
            "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141",
            605,
        )];

        for (i, (input, result)) in tests.iter().enumerate() {
            assert_eq!(&puzzle1(input), result, "Test {} failed", i);
        }
    }

    // #[test]
    // fn puzzle2_test() {
    //     let tests = [
    //         ("\"\"", 4),
    //         ("\"abc\"", 4),
    //         ("\"aaa\\\"aaa\"", 6),
    //         ("\"\\x27\"", 5),
    //         ("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"", 19),
    //     ];

    //     for (i, (input, result)) in tests.iter().enumerate() {
    //         assert_eq!(&puzzle2(input), result, "Test {} failed", i);
    //     }
    // }
}
