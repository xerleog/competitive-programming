impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows_ones: Vec<i32> = grid.iter().map(|r| r.iter().sum()).collect();
        let row_len = grid.len() as i32;

        let cols_ones: Vec<i32> = (0..grid[0].len())
            .map(|i| grid.iter().map(|r| r[i]).sum())
            .collect();
        let col_len = grid[0].len() as i32;
        let grid_val = row_len + col_len;

        grid.iter_mut().enumerate().for_each(|(i, r)| {
            r.iter_mut()
                .enumerate()
                .for_each(|(j, v)| *v = 2 * (rows_ones[i] + cols_ones[j]) - grid_val)
        });
        grid
    }
}
