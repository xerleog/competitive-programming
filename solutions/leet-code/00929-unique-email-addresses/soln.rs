use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut ans = HashSet::new();
        for i in emails
        {
            let temp = i.split('@').collect::<Vec<_>>();
            let left = temp[0].chars().take_while(|&c| c!= '+').filter(|&c| c!='.').collect::<String>();
            ans.insert([&left,temp[1]].join("@"));   
        }
        ans.len() as i32
    }
}
