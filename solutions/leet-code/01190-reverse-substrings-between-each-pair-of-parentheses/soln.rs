impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ans = vec![];
        for i in s.chars()
        {
            if i == ')'
            {
                let mut temp = vec![];
                let mut val = ans.pop();
                while val!= Some('(')
                {
                    temp.push(val.unwrap());
                    val = ans.pop();
                }
                ans.extend(temp);
            }
            else
            {   ans.push(i);}
        }
        ans.into_iter().collect::<String>()
    }
}
