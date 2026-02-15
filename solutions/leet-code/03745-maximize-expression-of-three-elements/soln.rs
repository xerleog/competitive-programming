impl Solution {
    pub fn maximize_expression_of_three(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        nums[n-1]+nums[n-2]-nums[0]
    }
}
