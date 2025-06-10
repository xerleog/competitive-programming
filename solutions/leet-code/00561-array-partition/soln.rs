impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        (0..nums.len()).filter(|x| x%2==0).map(|x| nums[x]).sum::<i32>()
    }
}
