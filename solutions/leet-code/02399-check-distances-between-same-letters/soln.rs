impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut ans = vec![0;26];
        let mut sol = s.as_bytes().into_iter().enumerate().map(|(a,b)| (b,a)).collect::<Vec<_>>();
        sol.sort();
        sol.chunks(2).all(|x| distance[(x[0].0-97)as usize] as usize==x[1].1-x[0].1-1)
    }
}
