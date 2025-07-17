impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let (mut m,mut t,mut ans)=(0,n,0);
        while t>1
        {
            if t&1==0
            {
                m = t/2;
                ans+=m;
                t = m;
            }
            else
            {   m = (t-1)/2;
                ans+=m;
                t = m+1;
            }
        }
        ans
    }
}
