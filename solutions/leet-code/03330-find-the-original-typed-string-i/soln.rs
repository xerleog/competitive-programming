impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        word.as_bytes()
            .chunk_by(|a, b| a == b)
            .fold(1, |acc, chunk| acc + chunk.len() as i32 - 1)
    }
}
