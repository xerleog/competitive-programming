impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let (mut ans, mut remaining) = (0, 0);
        for b in s.into_bytes() {
            if b == b'X' {
                if remaining == 0 {
                    ans += 1;
                    remaining = 3;
                }
            }
            if remaining > 0 {
                remaining -= 1;
            }
        }
        ans
    }
}
