impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        nums.iter().scan(0, |k, &n| {*k = (*k * 2 + n) % 5; Some(*k == 0)}).collect()
    }
}
