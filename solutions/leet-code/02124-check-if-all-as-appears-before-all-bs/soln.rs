impl Solution {
    pub fn check_string(s: String) -> bool {
        let val = s.find('b').unwrap_or(101);
        s.chars().enumerate().filter(|(_,y)| *y=='a').map(|x| x.0).all(|c| c<val)
    }
}
