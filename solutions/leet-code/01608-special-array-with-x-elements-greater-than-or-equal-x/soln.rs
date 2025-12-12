impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.sort_unstable();

        if nums[0] >= n as i32 {
            n as i32
        } else {
            let mut r = -1;

            for i in 1..n {
                let d = (n - i) as i32;

                if nums[i] >= d && nums[i-1] < d {
                    r = d;

                    break;
                }
            }

            r
        }
    }
}
