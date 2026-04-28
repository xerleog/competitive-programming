impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let res = nums.iter().fold([0, 1, 1], |[n, x, y], &z| {
            if x == 0 {
                [n + 1, y ^ 1, z ^ 1]
            } else {
                [n, y, z]
            }
        });

        if let [n, 1, 1] = res { n } else { -1 }
    }
}
