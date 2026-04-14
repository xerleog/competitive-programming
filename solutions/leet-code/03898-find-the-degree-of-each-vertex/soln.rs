impl Solution {
    pub fn find_degrees(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        matrix.into_iter().map(|x| x.into_iter().sum::<i32>()).collect()
    }
}
