impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut result = 0;
        for a in 1..=n {
            for b in 1..=n {
                let sum = a * a + b * b;
                let root = sum.isqrt();
                if root <= n && root * root == sum {
                    result += 1;
                }
            }
        }
        result
    }
}
