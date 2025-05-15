impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let (mut ans,mut val) = (vec![words[0].clone()],groups[0]);
        for i in 1..words.len()
        {
            if  groups[i]!=val
            {
                ans.push(words[i].clone());
                val = groups[i];
            }
        }
        ans
    }
}
