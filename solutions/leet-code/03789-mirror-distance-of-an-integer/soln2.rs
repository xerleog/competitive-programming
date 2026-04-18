impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let answer = n.to_string().chars().rev().fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as i32) - n;
        answer.abs()
    }
}
