impl Solution {
    pub fn has_special_substring(s: String, k: i32) -> bool {
        let k = k as usize;
        let bytes = s.as_bytes();
        bytes.windows(k)
            .enumerate()
            .filter(|(i, w)| w.iter().all(|b| *b == w[0]) && 
                             (*i == 0 || bytes[i-1] != w[0]) &&
                             (i + k >= bytes.len() || bytes[i+k] != w[0]))
            .next()
            .is_some()
    }
}
