impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut ans = vec![];
        while n>0
        {
            ans.push(n%10);
            n/=10;
        }
        ans.sort();
        let n = ans.len();
        ans[n-1]*ans[n-2]
    }
}
