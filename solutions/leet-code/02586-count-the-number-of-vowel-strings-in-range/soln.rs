impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        (left..=right).filter(|&x|  "aeiou".contains(words[x as usize].chars().next().unwrap())&& "aeiou".contains(words[x as usize].chars().last().unwrap())).count() as i32
    }
}
