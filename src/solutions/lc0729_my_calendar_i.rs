//! <https://leetcode.com/problems/my-calendar-i/>

use std::collections::BTreeSet;

#[derive(Default)]
struct MyCalendar {
    events: BTreeSet<(i32, i32)>,
}

/// time-complexity : O(log(n))
/// space-complexity : O(n)
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let ok_after = self
            .events
            .range((start, 0)..)
            .next()
            .map_or(true, |&(s, _)| s >= end);
        let ok_before = self
            .events
            .range(..(start, 0))
            .next_back()
            .map_or(true, |&(_, e)| e <= start);
        ok_before && ok_after && self.events.insert((start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut my_calendar = MyCalendar::new();
        assert!(my_calendar.book(10, 20));
        assert!(!my_calendar.book(15, 25));
        assert!(my_calendar.book(20, 30));
    }
}
