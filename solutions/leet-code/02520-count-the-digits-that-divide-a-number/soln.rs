impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let (mut temp,mut ans) = (num.clone(),0);
        while temp>0
        {
            if num%(temp%10)==0
            {
                ans+=1;
            }
            temp/=10;
        }
        ans
    }
}
