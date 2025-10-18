use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = HashMap::new();
        for (i,v) in nums.into_iter().enumerate()
        {
            match ans.get(&(target-v))
            {
                Some(&i2) => return vec![i as i32,i2],
                None => ans.insert(v,i as i32),
            };
        }
        vec![]
    }
}
