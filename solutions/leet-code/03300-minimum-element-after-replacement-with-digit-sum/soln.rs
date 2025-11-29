impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut ans = vec![];
        for i in nums
        {
            let (mut temp,mut val) =(i,0);
            while temp>0
            {
                val+=temp%10;
                temp/=10;
            }
            ans.push(val);
        }
        ans.into_iter().min().unwrap()
    }
}
