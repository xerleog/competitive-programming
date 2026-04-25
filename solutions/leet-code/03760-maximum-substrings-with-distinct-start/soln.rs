impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let (mut ans,mut sol) = (0,"".to_string());
        s.chars().for_each(|x|{ if !sol.contains(x) { ans+=1;} sol.push(x) });
        ans
    }
}
