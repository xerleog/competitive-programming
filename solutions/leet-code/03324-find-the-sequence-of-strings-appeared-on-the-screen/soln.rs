impl Solution {
    pub fn string_sequence(target: String) -> Vec<String> {
        let mut cur = "a".to_string();
        let mut ans = vec![cur.clone()];
        while cur != target
        {
            if target.starts_with(&cur)
            {
                cur.push('a');
            }
            else
            {
                let last = cur.pop().unwrap() as u8;
                cur.push((last+1) as char);
            }
            ans.push(cur.clone());
        }
        ans
    }
}
