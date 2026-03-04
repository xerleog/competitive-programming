impl Solution {
    pub fn trim_trailing_vowels(mut s: String) -> String {
        while !s.is_empty() && "aeiou".contains(s.chars().last().unwrap())
        {
            s.pop();
        }
        s
    }
}
