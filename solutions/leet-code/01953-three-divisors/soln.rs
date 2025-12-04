impl Solution {
    pub fn is_three(n: i32) -> bool {
        (2..=n/2).filter(|num| n % num == 0).take(2).count() == 1
    }
}
