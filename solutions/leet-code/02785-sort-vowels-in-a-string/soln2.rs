impl Solution {
    pub fn sort_vowels(s: String) -> String {
       let mut val = s.chars().filter(|&x| "AEIOUaeiou".contains(x)).collect::<Vec<_>>();
       val.sort();
       let mut val_iter = val.into_iter();
       s.chars().map(|x| if "AEIOUaeiou".contains(x) { val_iter.next().unwrap()} else { x}).collect()
    }
}
