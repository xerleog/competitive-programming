impl Solution {
    pub fn thousand_separator(mut n: i32) -> String {
        n.to_string().chars().collect::<Vec<_>>().rchunks(3).rev().map(|x| x.iter().collect::<String>()).collect::<Vec<_>>().join(".")
    }
}
