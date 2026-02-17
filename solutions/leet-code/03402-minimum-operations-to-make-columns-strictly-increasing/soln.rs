impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let mut total = 0;
        for j in 0..grid[0].len() {
            let mut x = grid[0][j] + 1;
            for i in 1..grid.len() {
                total += 0.max(x-grid[i][j]);
                x = x.max(grid[i][j]) + 1;
            }
        }
        total
    }
}
