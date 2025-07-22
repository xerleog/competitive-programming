impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        nums.into_iter().zip(index.into_iter()).for_each(|(a,b)| ans.insert(b as usize,a));
        ans   
    }
}
