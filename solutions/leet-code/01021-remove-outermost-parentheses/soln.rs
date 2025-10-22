impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let (mut ans, mut val) = ("".to_string(),0);
        for i in s.chars()
        {
            if i == '(' 
            {
                if val > 0 { ans.push(i);}
                val+=1;
            }
            else
            {
                val-=1;
                if val > 0 { ans.push(i);}
            }
        }
        ans
    }
}
