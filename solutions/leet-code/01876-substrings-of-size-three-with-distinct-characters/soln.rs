impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.as_bytes().windows(3).filter(|x| x[0]!=x[1] && x[1]!=x[2] && x[0]!=x[2]).count() as i32
    }
}
