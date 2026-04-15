impl Solution {
    pub fn count_elements(mut nums: Vec<i32>) -> i32 {
        let (l, h) = nums
            .iter()
            .fold((i32::MAX, i32::MIN), |(l, h), &n| (l.min(n), h.max(n)));
        nums.into_iter().fold(0, |s, n| s + i32::from(n > l && n < h))
    }
}
