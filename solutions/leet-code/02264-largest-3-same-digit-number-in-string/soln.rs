impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.as_bytes().windows(3).filter(|x| x[0]==x[1] && x[1]==x[2]).map(|y| y[0]).max().map(|z| (z as char).to_string().repeat(3)).unwrap_or_default()
    }
}
