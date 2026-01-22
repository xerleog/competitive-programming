impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        while !nums.is_sorted() {
            let i = (1..nums.len())
                .min_by_key(|&k| nums[k] + nums[k - 1])
                .unwrap();
            nums[i - 1] += nums.remove(i);
            result += 1;
        }
        result
    }
}
