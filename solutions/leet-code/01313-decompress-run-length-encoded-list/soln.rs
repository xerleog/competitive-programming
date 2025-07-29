impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        nums.chunks(2).fold(vec![],|mut a,w| { a.extend(vec![w[1];w[0] as usize]); a})
    }
}
