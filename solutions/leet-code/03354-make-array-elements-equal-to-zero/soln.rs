impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let (mut l, mut r, mut ans)=(0,nums.clone().into_iter().sum::<i32>(),0);
        for i in nums
        {
            l+=i;
            r-=i;
            let d = (l-r).abs();
            if (i==0 && d<2) { ans+=2-d;}
        }
        ans
    }
}
