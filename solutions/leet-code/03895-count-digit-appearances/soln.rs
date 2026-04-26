impl Solution {
    pub fn count_digit_occurrences(nums: Vec<i32>, digit: i32) -> i32 {
        nums.into_iter().map(|x| x.to_string().chars().filter(|&y| y==(b'0'+digit as u8)as char).count() as i32).sum::<i32>()
    }
}
