impl Solution {
    pub fn largest_even(mut s: String) -> String {
        while s.chars().last() != Some('2') { s.pop(); if s.is_empty() { return s;}}   
        s
    }
}
