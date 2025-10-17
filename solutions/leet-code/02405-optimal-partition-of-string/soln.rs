impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let (mut ans,mut val) = (vec![],"".to_string());
        for i in s.chars()
        {
            if val.contains(i)
            {
                ans.push(val);
                val = format!("{}",i);
            }
            else
            {
                val.push(i);
            }
        }
        ans.push(val);
        ans.len() as i32

    }
}
