impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        nums.iter().enumerate().filter_map(|(i, x)|
               ( x > nums.get(i.wrapping_sub(k)).unwrap_or(&i32::MIN)
            && x > nums.get(i+k).unwrap_or(&i32::MIN))
                .then_some(x)).sum()
    }
}
