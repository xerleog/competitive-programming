impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut sum = nums.clone().into_iter().sum::<i32>();
        let n = nums.len();
        (0..nums.len()).into_iter().map(|x| {sum-=nums[x]; if nums[x] as f64 >(sum as f64/(n-(x+1)) as f64) {1} else {0}} ).filter(|&y| y==1).count() as i32
    }
}
