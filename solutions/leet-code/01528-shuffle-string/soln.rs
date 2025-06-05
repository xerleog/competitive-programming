impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ans = indices.iter().zip(s.chars()).collect::<Vec<_>>();
        ans.sort();
        ans.into_iter().map(|(_,a)| a).collect::<String>()
    }
}
