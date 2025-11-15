impl Solution {
  pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut bitvec: Vec<i32> = vec![0; 32];
    for num in nums {
      let mut num = num;
      for i in 0..32 {
        if num & 1 == 1 {
          bitvec[i] += 1;
        }
        num >>= 1;
      }
    }
    let mut result = 0;
    for i in 0..32 {
      if bitvec[i] >= k {
        result += 1 << i;
      }
    }
    result
  }
}
