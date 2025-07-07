impl Solution {
    pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![];203];
        let mut sol:Vec<Vec<i32>> = vec![];
        for i in 0..rows 
        {
            for j in 0..cols
            {
                ans[((i-r_center).abs()+(j-c_center).abs()) as usize].push([i,j]);
            }
        }
        for x in 0..=202 {
            for pair in &ans[x] {
                sol.push(vec![pair[0], pair[1]]);
            }
        }
        sol
    }
    
}
