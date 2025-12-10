impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        for idx in 1..complexity.len() {
            if complexity[idx] <= complexity[0] { return 0;}
        }
        let mut result = 1;
        for idx in 1..complexity.len() {
            result = (result * idx) % 1000000007;
        }
        result as i32
    }
}
