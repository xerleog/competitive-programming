impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let mut bytes = unsafe { s.as_bytes_mut() };
        for (i, chunk) in bytes.chunks_mut(k as usize).enumerate() {
            if i % 2 == 0 {
                chunk.reverse()
            }
        }
        s
    }
}
