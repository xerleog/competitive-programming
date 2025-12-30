impl Solution {
    pub fn minimum_flips(n: i32) -> i32 {
        format!("{:b}",n).chars().zip(format!("{:b}",n).chars().rev()).filter(|x| x.0!=x.1).count() as i32
    }
}
