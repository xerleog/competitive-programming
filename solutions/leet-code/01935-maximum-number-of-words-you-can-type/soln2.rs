impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        text.split_whitespace().map(|x| broken_letters.chars().fold(1,|a,c| if x.contains(c) { a&0 } else { a&1})).filter(|&y| y==1).count() as i32
    }
}
