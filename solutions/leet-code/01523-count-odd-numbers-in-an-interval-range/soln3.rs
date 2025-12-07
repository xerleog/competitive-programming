impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (low % 2 + high - low + high % 2) / 2
    }
}
