impl Solution {
    pub fn count_monobit(n: i32) -> i32 {
        let ans = vec![0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023];
        ans.into_iter().filter(|&x| x<=n).count() as i32
    }
}
