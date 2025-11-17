impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let ans=nums.into_iter().enumerate().filter(|(_,x)| *x==1).map(|(p,q)| p).collect::<Vec<_>>();
        ans.windows(2).all(|i| (i[1]-i[0]-1) as i32>=k)
        
    }
}
