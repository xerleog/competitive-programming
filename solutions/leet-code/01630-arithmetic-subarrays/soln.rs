impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];
        for i in 0..l.len()
        {
            let mut temp = nums[l[i]as usize..=r[i] as usize].to_vec();
            temp.sort();
            let d = temp[1]-temp[0];
            ans.push(temp.windows(2).all(|x| x[1]-x[0] == d));
        }
        ans
    }
}
