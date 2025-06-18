impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for i in 0..grid.len()
        {   grid[i].sort(); }
        let  mut ans = 0;
        for j in 0..grid[0].len()
        {
            let mut temp =0;
            for i in 0..grid.len()
            {
                temp = temp.max(grid[i][j]);
            }
            ans+=temp;
        }
        ans
    }
}
