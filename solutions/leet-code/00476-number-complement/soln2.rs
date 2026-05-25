impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut n = 1;
        while n < num {
            n = (n << 1) + 1;
        }
        n - num
    }
}
