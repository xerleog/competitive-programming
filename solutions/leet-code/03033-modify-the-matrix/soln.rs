impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = matrix.clone();
        let mut sol = vec![];
        for i in 0..matrix[0].len()
        {
            let mut ma = -2;
            for j in 0..matrix.len()
            {
                ma = ma.max(matrix[j][i]);
            }
            sol.push(ma);
        }
        for i in 0..matrix[0].len()
        {
            for j in 0..matrix.len()
            {
                if ans[j][i]==-1 { ans[j][i]=sol[i];}
            }
        }
        ans        
    }
}
