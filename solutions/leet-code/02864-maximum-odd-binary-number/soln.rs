impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort_by(|a,b| b.cmp(a));
        s.remove(0);
        s.push('1');
        s.into_iter().collect::<String>()
    }
}
