impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in words.into_iter()
        {
            let m = i.chars().rev().collect::<String>();
            if i==m
            {   return i;}
        }
        "".to_string()
    }
}
