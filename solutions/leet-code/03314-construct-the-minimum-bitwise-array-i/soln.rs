impl Solution {
  pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for n in nums {
      let mut found = false;
      for i in 0..n {
        let mut trial = i as i32 | (i+1) as i32;
        if (trial == n) {
          res.push(i);
          found = true;
          break
        }
      }
      if !found {
        res.push(-1);
      }
    }
    res
  }
}
