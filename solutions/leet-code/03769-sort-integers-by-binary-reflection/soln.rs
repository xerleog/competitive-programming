impl Solution {
    pub fn sort_by_reflection(nums: Vec<i32>) -> Vec<i32> {
       let mut nums = nums;
       nums.sort_by(|&x, &y| {
           let x = x as u32;
           let b = 32 - x.leading_zeros();
           let r = x.reverse_bits();
           let xr = r >> (32 - b);

           let y = y as u32;
           let b = 32 - y.leading_zeros();
           let r = y.reverse_bits();
           let yr = r >> (32 - b);

           if xr == yr {
               return x.cmp(&y);
           }
           xr.cmp(&yr)
        });

        nums.into_iter().collect()
    }
}
