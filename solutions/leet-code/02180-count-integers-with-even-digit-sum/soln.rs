impl Solution {
    pub fn count_even(num: i32) -> i32 {
        (2..=num).filter(|i| {
            let (mut sum, mut x) = (0, *i);
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            (sum & 1).eq(&0)
        }).count() as i32
    }
}
