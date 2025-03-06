impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut ans = vec![0;n*n];
        grid.into_iter().for_each(|x|{ x.into_iter().for_each(|y| {ans[(y-1) as usize]+=1;})});
        let sol = vec![
        ans.iter().position(|&y| y == 2).map_or(-1, |x| (x + 1) as i32),
        ans.iter().position(|&y| y == 0).map_or(-1, |x| (x + 1) as i32),
    ];
        sol
    }
}
