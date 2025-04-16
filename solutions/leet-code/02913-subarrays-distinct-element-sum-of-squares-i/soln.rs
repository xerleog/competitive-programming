use std::collections::HashSet;

impl Solution {
  pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let mut count = HashSet::new();
    let mut sum : i32 = 0;
    for i in 0..nums.len() {
      for j in i..nums.len() {
        count.insert(nums[j]);
        sum += count.len().pow(2) as i32;
      }
      count.clear();
    }
    sum
  }
}

