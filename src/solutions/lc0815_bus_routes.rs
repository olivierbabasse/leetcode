//! <https://leetcode.com/problems/bus-routes/>

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let routes = routes
            .into_iter()
            .map(|v| v.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let mut adj_list: HashMap<usize, Vec<usize>> = HashMap::new();
        for (index1, r1) in routes.iter().enumerate() {
            for (index2, r2) in routes.iter().enumerate().skip(index1 + 1) {
                if r1.intersection(r2).count() > 0 {
                    adj_list.entry(index1).or_default().push(index2);
                    adj_list.entry(index2).or_default().push(index1);
                }
            }
        }

        let mut queue = VecDeque::new();
        for (index, r) in routes.iter().enumerate() {
            if r.contains(&source) {
                queue.push_back(index);
            }
        }

        let mut visited = vec![false; routes.len()];
        let mut bus_count = 1;
        while !queue.is_empty() {
            let mut queue_len = queue.len();
            while queue_len > 0 {
                queue_len -= 1;
                let current_route = queue.pop_front().unwrap();

                if routes[current_route].contains(&target) {
                    return bus_count;
                }

                if let Some(adj_routes) = adj_list.get(&current_route) {
                    for &adj_route in adj_routes {
                        if !visited[adj_route] {
                            visited[adj_route] = true;
                            queue.push_back(adj_route);
                        }
                    }
                }
            }

            bus_count += 1;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::num_buses_to_destination(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6),
            2
        );
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![
                    vec![7, 12],
                    vec![4, 5, 15],
                    vec![6],
                    vec![15, 19],
                    vec![9, 12, 13]
                ],
                15,
                12
            ),
            -1
        );
        assert_eq!(
            Solution::num_buses_to_destination(
                vec![vec![1], vec![15, 16, 18], vec![10], vec![3, 4, 12, 14]],
                3,
                15
            ),
            -1
        );
    }
}
