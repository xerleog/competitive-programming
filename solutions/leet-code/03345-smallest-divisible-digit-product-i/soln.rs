impl Solution {
    pub fn prod(x:i32) -> i32{
        let (mut a,mut b) =(x,1);
        while a>0
        {
            b*=(a%10);
            a/=10;
        }
        return b;
    }
    pub fn smallest_number(mut n: i32, t: i32) -> i32 {
        while (Self::prod(n) % t !=0) { n+=1;}
        n
    }
}
