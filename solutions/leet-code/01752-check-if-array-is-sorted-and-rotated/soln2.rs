impl Solution {
    pub fn check(mut nums: Vec<i32>) -> bool {
        nums.push(nums[0]);
        nums.windows(2)
            .filter(|w| w[0] > w[1])
            .count() <= 1
    }
}
