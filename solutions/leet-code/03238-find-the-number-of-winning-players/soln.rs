impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        pick.into_iter()
            .fold(vec![[0; 11]; n as usize], |mut acc, p| {
                acc[p[0] as usize][p[1] as usize] += 1;
                acc
            })
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, p)| {
                if p.into_iter().any(|v| v > i) {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}
