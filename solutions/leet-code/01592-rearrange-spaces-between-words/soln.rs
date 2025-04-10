impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let cnt = text.clone().chars().filter(|&x| x==' ').count();
        let mut ans = text.split_whitespace().collect::<Vec<_>>();
        if ans.len() == 1 {
        return format!("{}{}", ans[0], " ".repeat(cnt));
        }
        format!("{}{}",ans.join(&" ".repeat(cnt/(ans.len()-1))),&" ".repeat(cnt%(ans.len()-1)))
        
    }
}
