impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        ((n+1)*n)/2-(m*((n/m)*(1+n/m)))
    }
}
