impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let (a,b) = (n/7,n%7);    
        28*a+7*a*(a-1)/2+b*(b+1)/2+b*a
    }
}
