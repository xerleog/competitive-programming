impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut curr = b'2';
        let mut count = 0;
        let mut prev = 0;
        let mut ans = 0;
        for b in s.bytes() {
            if b != curr {
                curr = b;
                prev = count;
                count = 0;
            }
            count += 1;
            if count <= prev {
                ans += 1;
            }
        }
        ans
    }
}
