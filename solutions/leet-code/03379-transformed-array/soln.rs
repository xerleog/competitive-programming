impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().enumerate().map(|(idx,&num)| nums[((idx as i32 + num as i32) % nums.len() as i32 + nums.len() as i32) as usize % nums.len()]).collect()
    }
}
