impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut f = [0; 101];
        let n = nums.len();

        for i in 0..n {
            let x = nums[i];
            f[x as usize] += 1;
        }

        let mut a = i32::MAX;
        let mut b = i32::MAX;

        for i in 0..n {
            let x = nums[i];
            for j in 0..n {
                let y = nums[j];
                if f[x as usize] != f[y as usize] {
                    if x < a {
                        a = x;
                        b = y;
                    } else if x == a {
                        if y < b {
                            b = y;
                        }
                    }
                }
            }
        }

        if a == i32::MAX {
            return vec![-1, -1];
        }
        let p = vec![a, b];

        p
    }
}
