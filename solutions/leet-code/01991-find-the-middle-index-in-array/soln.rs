impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let (mut val,mut sum) = (0,nums.clone().into_iter().sum::<i32>());
        for i in 0..nums.len() { if val==sum-val-nums[i] { return i as i32;} else { val+=nums[i];} }
        -1
    }
}
