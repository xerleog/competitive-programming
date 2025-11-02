impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut ans = vec![0;s.len()];
        let mut sol = s.chars().enumerate().filter(|x| x.1==c).map(|y| y.0).collect::<Vec<_>>();
        (0..s.len()).map(|x| sol.iter().map(|&y| x.abs_diff(y)).min().unwrap() as i32).collect::<Vec<_>>()
    }
}
