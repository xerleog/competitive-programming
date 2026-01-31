impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        letters.clone().into_iter().find(|&x| x>target).unwrap_or(letters[0])
    }
}
