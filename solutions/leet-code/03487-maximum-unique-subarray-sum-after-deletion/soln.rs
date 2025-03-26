impl Solution {
    pub fn max_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        if nums.iter().all(|&x|x<0) {
            return nums.into_iter().max().unwrap()
        }
        nums.into_iter().filter(|&x|x>0).sum::<i32>()
    }
}
