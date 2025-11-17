use itertools::Itertools;
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        nums.into_iter()
            .enumerate()
            .filter(|&(_, byte)| byte == 1)
            .map(|(i, _)| i)
            .tuple_windows()
            .all(move |(i, j)| j - i >= k as usize + 1)
    }
}
