impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
       let (l,n)=(word.chars().position(|x| x==ch).unwrap_or(0),word.len()); 
        if l==0
        {   return word;}
        format!("{}{}",&word[0..=l].chars().rev().collect::<String>(),&word[l+1..n])
    }
}
