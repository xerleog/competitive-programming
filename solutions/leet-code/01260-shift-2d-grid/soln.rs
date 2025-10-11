impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = grid.clone().into_iter().flat_map(|x| x.into_iter()).collect::<Vec<_>>();
        let n = ans.len();
        ans.rotate_right(k as usize % n);
        ans.chunks(grid[0].len()).map(|x| x.to_vec()).collect()
    }
}
