//! <https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/>

use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap},
};

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(n)
impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut available_servers = (0..k).collect::<BTreeSet<_>>();
        let mut request_count_by_server = vec![0; k];
        let mut running_requests: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

        for (request_counter, (time, load)) in arrival.into_iter().zip(load).enumerate() {
            while running_requests.peek().is_some() && running_requests.peek().unwrap().0 .0 <= time
            {
                let running_request = running_requests.pop().unwrap();
                available_servers.insert(running_request.0 .1);
            }

            if !available_servers.is_empty() {
                let next_server =
                    if let Some(&server) = available_servers.range(request_counter % k..).next() {
                        server
                    } else {
                        *available_servers.range(..).next().unwrap()
                    };
                available_servers.remove(&next_server);
                request_count_by_server[next_server] += 1;
                running_requests.push(Reverse((time + load, next_server)));
            }
        }

        let max_count = *request_count_by_server.iter().max().unwrap();
        request_count_by_server
            .into_iter()
            .enumerate()
            .filter(|(_, count)| *count == max_count)
            .map(|(index, _)| index as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4, 5], vec![5, 2, 3, 3, 3]),
            vec![1]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3, 4], vec![1, 2, 1, 2]),
            vec![0]
        );
        assert_eq!(
            Solution::busiest_servers(3, vec![1, 2, 3], vec![10, 12, 11]),
            vec![0, 1, 2]
        );
    }
}
