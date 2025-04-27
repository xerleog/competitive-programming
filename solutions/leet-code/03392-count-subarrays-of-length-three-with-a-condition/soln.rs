impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3).filter(|w| 2*(w[0]+w[2])==w[1]).count() as i32
    }
}
