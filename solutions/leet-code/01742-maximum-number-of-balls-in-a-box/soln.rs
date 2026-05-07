impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut ans = vec![0;46];
        for mut i in low_limit..=high_limit
        {
            let mut temp = 0;
            while i>0
            {
                temp+=i%10;
                i/=10;
            }
            ans[temp as usize]+=1;
        }
        ans.into_iter().max().unwrap()
    }
}
