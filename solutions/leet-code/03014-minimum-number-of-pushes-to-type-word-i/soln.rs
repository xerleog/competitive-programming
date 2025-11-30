impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut k = word.len();
        let mut total = 0;
        for i in &[5,4,3,2] {
            let b = 8*(i-1);
            if k > b {
                total += (k-b)*i;
                k = b;
            }
        }
        (k + total) as _
    }
}
