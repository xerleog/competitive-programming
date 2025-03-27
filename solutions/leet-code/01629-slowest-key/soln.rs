impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut ans = release_times.windows(2).map(|w| w[1]-w[0]).collect::<Vec<_>>();
        ans.insert(0,release_times[0]);
        let mut sol = ans.iter().zip(keys_pressed.chars()).collect::<Vec<_>>();
        sol.sort();
        sol.last().unwrap().1
    }
}
