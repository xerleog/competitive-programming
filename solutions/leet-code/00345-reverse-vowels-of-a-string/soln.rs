impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut ans = s.clone().chars().filter(|&x| "aeiouAEIOU".contains(x)).collect::<Vec<_>>();
        s.chars().map(|x| if "aeiouAEIOU".contains(x) { ans.pop().unwrap()} else { x}).collect::<String>()
    }
}
