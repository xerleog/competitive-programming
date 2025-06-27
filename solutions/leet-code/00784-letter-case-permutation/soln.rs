use std::collections::BTreeSet;
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = BTreeSet::new();
        for i in 0..(1<<s.len())
        {
            let mut temp = s.chars().collect::<Vec<_>>();
            for j in 0..s.len()
            {
                if i&(1<<j)!=0
                {
                    let n = temp[j] as u8;
                    if n>=65 && n<=90
                    {   temp[j]= (n+32) as char;}
                    else if n>=97 && n<=122
                    {   temp[j]= (n-32) as char;}
                }
            }
            ans.insert(temp.into_iter().collect::<String>());
        }
        ans.into_iter().collect()
    }
}
