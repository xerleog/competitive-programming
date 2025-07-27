impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut ans = vec![];
        while num>0
        {
            ans.push(num%10);
            num/=10;
        }
        ans.sort();
        (ans[0]+ans[1])*10+(ans[2]+ans[3])
    }
}
