impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut ans = vec![0;501];
        nums.into_iter().for_each(|x| ans[x as usize]+=1);
        ans.into_iter().all(|x| x%2==0)
    }
}
