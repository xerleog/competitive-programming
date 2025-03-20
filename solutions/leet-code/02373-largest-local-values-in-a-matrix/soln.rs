impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans : Vec<Vec<i32>> = Vec::new();

        for i in 0..(grid.len()-2) {
            let mut rows = Vec::new();
            for j in 0..(grid.len()-2){
                
                let mut max = grid[i][j];

                for k in 0..3{
                    for l in 0..3{
                        if grid[i+k][j+l] > max {
                            max = grid[i+k][j+l];
                        }
                    }
                }
                rows.push(max);
            }
            ans.push(rows);
        }

        return ans;
    }
}
