impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        format!("{}{}",s[..k as usize].chars().rev().collect::<String>(),&s[k as usize..])
    }
}
