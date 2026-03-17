use std::collections::HashMap;

impl Solution {
    pub fn first_unique_even(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f: HashMap<i32, usize> = HashMap::new();

        for i in 0..n {
            let x = nums[i];
            if x % 2 == 0 {
                *f.entry(x).or_default() += 1;
            }
        }

        for i in 0..n {
            let x = nums[i];
            match f.get(&x) {
                Some(&v) => {
                    if v == 1 {
                        return x;
                    }
                }
                _ => {}
            }
        }

        -1
    }
}
