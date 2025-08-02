impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        (0..mat.len()).fold(0,|a,c| a+mat[c][c]+mat[c][mat.len()-c-1])-(mat.len() as i32&1)*(mat[mat.len()/2][mat.len()/2])
    }
}
