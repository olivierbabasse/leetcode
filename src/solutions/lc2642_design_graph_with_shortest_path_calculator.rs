//! <https://leetcode.com/problems/design-graph-with-shortest-path-calculator/>

use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Debug)]
struct Edge {
    node: i32,
    weight: i32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| other.node.cmp(&self.node))
    }
}
struct Graph {
    adj_list: HashMap<i32, Vec<Edge>>,
    count: usize,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut adj_list: HashMap<i32, Vec<Edge>> = HashMap::new();
        for e in &edges {
            adj_list.entry(e[0]).or_default().push(Edge {
                node: e[1],
                weight: e[2],
            });
        }
        Self {
            adj_list,
            count: n as usize,
        }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.adj_list.entry(edge[0]).or_default().push(Edge {
            node: edge[1],
            weight: edge[2],
        });
    }

    /// time-complexity : O(edges*log(vertices))
    /// space-complexity : O(vertices)
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        const MAX: i32 = i32::MAX / 2;
        let mut distances = vec![MAX; self.count];
        distances[node1 as usize] = 0;

        let mut priority_queue = BinaryHeap::<Edge>::new();
        priority_queue.push(Edge {
            node: node1,
            weight: 0,
        });
        while let Some(top_node) = priority_queue.pop() {
            if let Some(adj_edges) = self.adj_list.get(&top_node.node) {
                for edge in adj_edges {
                    if distances[edge.node as usize]
                        > distances[top_node.node as usize] + edge.weight
                    {
                        distances[edge.node as usize] =
                            distances[top_node.node as usize] + edge.weight;
                        priority_queue.push(Edge {
                            node: edge.node,
                            weight: distances[edge.node as usize],
                        });
                    }
                }
            }
        }

        if distances[node2 as usize] == MAX {
            -1
        } else {
            distances[node2 as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_1() {
        let mut obj = Graph::new(
            4,
            vec![vec![0, 2, 5], vec![0, 1, 2], vec![1, 2, 1], vec![3, 0, 3]],
        );
        assert_eq!(obj.shortest_path(3, 2), 6);
        assert_eq!(obj.shortest_path(0, 3), -1);
        obj.add_edge(vec![1, 3, 4]);
        assert_eq!(obj.shortest_path(0, 3), 6);
    }

    #[test]
    fn test_cases_2() {
        let obj = Graph::new(
            6,
            vec![
                vec![3, 5, 990551],
                vec![1, 3, 495721],
                vec![0, 1, 985797],
                vec![4, 5, 422863],
                vec![4, 1, 505663],
            ],
        );
        assert_eq!(obj.shortest_path(0, 1), 985797);
        assert_eq!(obj.shortest_path(3, 5), 990551);
        assert_eq!(obj.shortest_path(4, 4), 0);
        assert_eq!(obj.shortest_path(0, 3), 1481518);
    }
}
