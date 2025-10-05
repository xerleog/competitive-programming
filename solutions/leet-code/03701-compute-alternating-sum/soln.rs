impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        nums.chunks(2).map(|x| if x.len()<2 {x[0]} else {x[0]-x[1]}).sum::<i32>()
    }
}
