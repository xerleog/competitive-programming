impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let (mut m,mut a,mut b)=(n,0,1);
        while m>0 
        { 
            let d = m%10;
            a+=d;
            b*=d;
            m/=10;
        }
        n%(a+b)==0
    }
}
