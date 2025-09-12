impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        s.bytes().any(|b| b"aeiou".contains(&b))
    }
}
