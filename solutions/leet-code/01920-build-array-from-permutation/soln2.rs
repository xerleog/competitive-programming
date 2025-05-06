impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).map(|x| nums[nums[x] as usize]).collect::<Vec<_>>()
    }
}
