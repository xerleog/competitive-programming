impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut x2,mut x1,mut y2, mut y1)=(0,i32::MAX,0,i32::MAX);
        for i in (0..grid.len())
        {
            for j in (0..grid[0].len())
            {
                if grid[i][j]==1
                {
                    x2=x2.max(i as i32);
                    y2=y2.max(j as i32);
                    x1=x1.min(i as i32);
                    y1=y1.min(j as i32);
                }
            }
        }
        (x2-x1+1)*(y2-y1+1)
    }
}
