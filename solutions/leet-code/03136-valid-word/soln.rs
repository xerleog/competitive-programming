impl Solution {
    pub fn is_valid(word: String) -> bool {
        const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u'];
        const CONSONANTS: &[char] = &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w','x', 'y', 'z'];
        let word = word.to_lowercase();
        word.len() >= 3
            && word.chars().all(char::is_alphanumeric)
            && word.contains(VOWELS)
            && word.contains(CONSONANTS)
    }
}
