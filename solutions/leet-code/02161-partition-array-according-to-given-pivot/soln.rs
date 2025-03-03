impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut ans = vec![];
        ans.extend(nums.clone().into_iter().filter(|&x| x<pivot));
        ans.extend(nums.clone().into_iter().filter(|&x| x==pivot));
        ans.extend(nums.clone().into_iter().filter(|&x| x>pivot));
        ans
    }
}
