impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len()/2;
        let mut hash = vec![0;10000];
        for num in nums {
            hash[num as usize] += 1;
            if hash[num as usize] == n { return num}
        }
        0
    }
}
