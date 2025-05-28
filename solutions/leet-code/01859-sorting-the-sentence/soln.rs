impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut s = s.split_whitespace().collect::<Vec<_>>();
        s.sort_by(|a,b| a.chars().last().cmp(&b.chars().last()));
        let ans = s.into_iter().map(|x| format!("{} ", &x[..x.len() - 1])).collect::<String>();
        ans[..ans.len()-1].to_string()
    }
}
