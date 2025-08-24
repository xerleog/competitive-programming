impl Solution {
    pub fn even_odd_bit(mut n: i32) -> Vec<i32> {
        let mut ans = vec![0,0];
        let mut q = true;
        while n>0
        {
            if n&1==1
            {
                if q { ans[0]+=1;} else { ans[1]+=1;}
            }
            q^=true;
            n=n>>1;
        }
        ans
    }
}
