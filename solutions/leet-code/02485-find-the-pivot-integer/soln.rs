impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let m = ((n*n+n)/2) as f64; 
        let temp = m.sqrt() as i32;
        if temp*temp==m as i32{   temp} else {   -1}
    }
}
