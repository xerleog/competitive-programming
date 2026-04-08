impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.clone().into_iter().all(|x| x==nums[0]) { 0 } else { 1 } 
    }
}
