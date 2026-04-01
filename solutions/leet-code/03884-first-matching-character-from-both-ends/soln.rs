impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let n = s.len();
        let cs = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i <= (n - i - 1) {
            if cs[i] == cs[n - i - 1] {
                return i as i32;
            }
            i += 1; 
        }
        
        -1
    }
}
