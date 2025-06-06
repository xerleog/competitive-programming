impl Solution {
    pub fn replace_digits(s: String) -> String {
       let mut ans = String::new();
       for i in s.as_bytes().chunks(2)
       {
            ans.push(i[0] as char);
            if i.len()==2
            {   ans.push((i[0]+i[1]-48)as char);}
       }
        ans
    }
}
