impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (a,b) = s.split_at(s.len()/2);
        a.chars().filter(|&x| "aeiouAEIOU".contains(x)).count()==b.chars().filter(|&x| "aeiouAEIOU".contains(x)).count()
    }
}
