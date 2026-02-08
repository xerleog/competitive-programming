impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let (mut a,mut b) = (vec![],vec![]);
        s.clone().chars().for_each(|x| if x.is_lowercase() { a.push(x);} else { b.push(x);});
        s.chars().map(|x| if x.is_lowercase() { a.pop().unwrap()} else { b.pop().unwrap()}).collect::<String>()

    }
}
