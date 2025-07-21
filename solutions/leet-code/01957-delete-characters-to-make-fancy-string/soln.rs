impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        if s.len()<2 { return s;}
        let mut ans = String::new();
        let s = s.chars().collect::<Vec<_>>();
        for i in s.windows(3)
        {
            if i[0]==i[1] && i[1]==i[2]
            {   continue;}
            else
            {   ans.push(i[0]);}
        }
        ans.push(s[s.len()-2]);
        ans.push(s[s.len()-1]);
        ans
       
    }
}
