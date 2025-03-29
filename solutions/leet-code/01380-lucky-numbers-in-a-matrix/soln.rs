impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        matrix
            .iter()
            .map(|row| (row.iter().enumerate().map(|(i, v)| (*v, i)).min().unwrap()))
            .max()
            .map(|(v, col)| {
                if matrix.iter().any(|row| row[col] > v) {
                    vec![]
                } else {
                    vec![v]
                }
            })
            .unwrap()
    }
}
