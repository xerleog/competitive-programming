impl Solution {
    pub fn subarray_sum(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold(0,|a,x| 
        a+nums[0.max(x as i32-nums[x]) as usize..=x].iter().sum::<i32>()
        )
    }
}
