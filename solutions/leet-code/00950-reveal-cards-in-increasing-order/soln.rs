use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut deck = deck.into_iter();

        let mut queue = VecDeque::from_iter(0..deck.len());
        let mut result = vec![0; deck.len()];

        while let Some(i) = queue.pop_front() {
            result[i] = deck.next().unwrap();
            queue.rotate_left(1.min(queue.len()));
        }

        result
    }
}
