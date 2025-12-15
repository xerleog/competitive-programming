impl Solution {
    pub fn max_k_distinct(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        nums.sort_by(|a,b| b.cmp(&a));
        nums.dedup();
        nums[..nums.len().min(k as usize)].to_vec()
    }
}
