impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0;nums.len()];
        let (mut i,mut j)=(0,1);
        for k in nums.into_iter()
        {
            if k&1==0 { ans[i]+=k;i+=2;}
            else { ans[j]+=k;j+=2;}
        }
        ans
    }
}
