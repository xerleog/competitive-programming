impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;nums.len()];
        for i in nums.clone()
        {   ans[i as usize]=nums[nums[i  as usize] as usize] as i32;}
        ans
    }
}
