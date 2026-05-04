impl Solution {
    pub fn vowel_consonant_score(s: String) -> i32 {
        let mut cs = 0;
        let mut vs = 0;
        for c in s.chars() {
            if c == ' ' || c.is_digit(10) { continue; }
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vs += 1;
            } else {
                cs += 1;
            }
        }
        if cs > 0 {
            let score = (vs as f64 / cs as f64).floor() as i32;
            return score
        }
        0
    }
}
