impl Solution {
    pub fn abs_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let (k,n)=(k as usize,nums.len());
        nums.sort();
        (&nums[..k].into_iter().sum::<i32>() - &nums[n-k..].into_iter().sum::<i32>()).abs()
    }
}
