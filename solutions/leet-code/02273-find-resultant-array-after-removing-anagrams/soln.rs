impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut last = words[0].clone();
        let mut ans = vec![last.clone()];
        for i in words.into_iter().skip(1)
        {
            let (mut temp1,mut temp2)=(last.chars().collect::<Vec<_>>(),i.chars().collect::<Vec<_>>());
            temp1.sort();
            temp2.sort();
            if temp1 != temp2
            {
                last = i;
                ans.push(last.clone());
            }
        }
        ans
    }
}
