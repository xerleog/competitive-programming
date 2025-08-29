impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let ans = sentence.split_whitespace().collect::<Vec<_>>();
        let mut solve = vec![];
        for i in 0..ans.len()
        {
            if "aeiouAEIOU".contains(ans[i].chars().next().unwrap())
            {
                solve.push(format!("{}{}{}",ans[i],"ma","a".repeat(i+1)));
            }
            else
            {
                solve.push(format!("{}{}{}{}",&ans[i][1..],&ans[i][..1],"ma","a".repeat(i+1)));
            }
        }
        solve.join(" ")
    }
}
