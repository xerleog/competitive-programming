impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        mat.iter()
            .all(|row| row.iter()
            .zip(row.iter().chain(row.iter()).skip(k as usize % mat[0].len()))
            .all(|(a,b)| a == b)
        )
    }
}
