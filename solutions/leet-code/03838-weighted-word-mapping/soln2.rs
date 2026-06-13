impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .into_iter()
            .map(|w| w.bytes().map(|b| weights[(b - b'a') as usize]).sum())
            .map(|b: i32| (122 - b % 26) as u8 as char)
            .collect()
    }
}
