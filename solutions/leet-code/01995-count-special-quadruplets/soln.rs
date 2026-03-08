impl Solution {
  pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let sz = nums.len();
    let mut sorted = nums.clone();
    sorted.sort();
    let mut res = 0;
    for i in 0..sz-3 {
      for j in i+1..sz-2 {
        for k in j+1..sz-1 {
          for l in k+1..sz {
            if nums[i]+nums[j]+nums[k] == nums[l] {
              res+=1;
            }
          }
        }
      }
    }
    res
  }
}
