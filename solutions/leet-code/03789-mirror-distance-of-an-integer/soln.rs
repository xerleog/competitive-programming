impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut m =n.to_string().chars().rev().collect::<String>();
        (n-m.parse::<i32>().unwrap()).abs()
    }
}
