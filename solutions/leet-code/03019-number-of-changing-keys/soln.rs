impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        s.to_lowercase().as_bytes().windows(2).filter(|x| x[0]!=x[1]).count() as i32
    }
}
