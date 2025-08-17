impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut val = 0;
        let mut ans = 0;
        for i in s.chars()
        {
            if i=='E'
            {
                val+=1;
            }
            else
            {
                val-=1;
            }
            ans = ans.max(val);
        }
        ans
    }
}
