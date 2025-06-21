impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut counter: i32 = 0;
      for i in 0..=limit {
          for j in 0..=limit {
              let k = n - i - j as i32;
              if 0 <= k && k <= limit {
                  counter += 1;
              }
          }
      }
      counter 
    }
}
