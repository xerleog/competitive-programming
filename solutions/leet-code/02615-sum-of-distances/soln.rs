use std::collections::HashMap;
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut ans = vec![0i64; n];
        let mut hash = HashMap::<i32, (i64, i64)>::new();
        for i in 0..n {
            let entry = hash.entry(nums[i]).or_insert((0, 0));
            ans[i] = i as i64 * entry.0 - entry.1;
            entry.0 += 1;
            entry.1 += i as i64;
        }
        for i in 0..n {
            let entry = hash.entry(nums[i]).or_insert((0, 0));
            entry.0 -= 1;
            entry.1 -= i as i64;
            ans[i] += entry.1 - i as i64 * entry.0;
        }
        ans
    }
}
