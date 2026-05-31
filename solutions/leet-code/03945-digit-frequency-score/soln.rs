impl Solution {
    pub fn digit_frequency_score(n: i32) -> i32 {
        n.to_string().as_bytes().into_iter().map(|x| (x-48) as i32).sum::<i32>()
    }
}
