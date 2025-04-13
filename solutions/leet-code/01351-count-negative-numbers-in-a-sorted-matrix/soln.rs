impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.into_iter().flat_map(|x| x.into_iter()).filter(|&w| w<0).count() as i32
    }
}
