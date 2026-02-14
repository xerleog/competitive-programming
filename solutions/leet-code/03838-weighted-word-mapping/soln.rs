impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words.into_iter().map(|x| (((122- x.as_bytes().into_iter().map(|y| weights[(y-97) as usize]).sum::<i32>()%26) as u8) as char)).collect::<String>()  
    }
}
