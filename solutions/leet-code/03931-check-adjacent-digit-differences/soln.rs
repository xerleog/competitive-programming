impl Solution {
    pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
        s.as_bytes().windows(2).all(|x| (x[0] as i32-x[1] as i32).abs()<=2)
    }
}
