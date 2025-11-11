impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        n.to_string().replace("0","").parse::<i64>().expect("REASON")
    }
}
