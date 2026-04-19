impl Solution {
    pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
        let (n,m,k)=(n as i64,m as i64,k as i64);
        ((n-k).max(0)+(m-k).max(0))*k
    }
}
