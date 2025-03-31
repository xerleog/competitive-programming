impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.as_bytes().into_iter().enumerate().fold(0,|a,(b,c)| a+((123-c) as i32*(b+1) as i32))   
    }
}
