impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
       let n = grid.len();
       let mut sol = vec![0;n*n];
       let mut ans = vec![0,0];
       for i in 0..n
        {
            for j in 0..n
            {
                if(sol[(grid[i][j]-1) as usize]==0)
                {   sol[(grid[i][j]-1) as usize]+=1;}
                else
                {   ans[0]=grid[i][j];}
            }
        }
        for i in 0..n*n
        {   
            if sol[i]==0
            {
                ans[1]=(i+1) as i32;
            }
        }
        ans
    }
}
