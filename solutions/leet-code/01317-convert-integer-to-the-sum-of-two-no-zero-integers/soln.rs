impl Solution {
    pub fn check(mut x: i32) -> bool {
        let mut val = true;
        while x>0{
            if x%10==0{
                val=false;
                break;
            }
            else { x/=10;}
        }
        return val;
    }
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n
        {
            if Self::check(i) && Self::check(n-i)
            {
                return vec![i,n-i];
            }
        }
        vec![]
    }
}
