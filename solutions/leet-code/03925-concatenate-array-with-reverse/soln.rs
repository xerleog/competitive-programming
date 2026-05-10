impl Solution {
    pub fn concat_with_reverse(mut nums: Vec<i32>) -> Vec<i32> {
      nums.extend(nums.clone().into_iter().rev().collect::<Vec<_>>());
      nums
    }
}
