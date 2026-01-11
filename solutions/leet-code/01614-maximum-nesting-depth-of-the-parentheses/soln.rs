impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut n,mut ma)=(0,0);
        for i in s.chars()
        {
            if i == '(' { n+=1;}
            else if i == ')' { n-=1;}
            ma = ma.max(n);
        }   
        ma
    }
}
