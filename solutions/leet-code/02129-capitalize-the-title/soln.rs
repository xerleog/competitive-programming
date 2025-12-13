impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let mut ans = title.split_whitespace().map(|x| x.to_lowercase()).collect::<Vec<_>>();
        ans.iter_mut().for_each(|x| if x.len() > 2 { *x = format!("{}{}", x.chars().next().unwrap().to_uppercase(), &x[1..]); });
        ans.join(" ")
    }
}
