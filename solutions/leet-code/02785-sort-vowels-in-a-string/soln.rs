impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut val = s.chars().filter(|&x| "AEIOUaeiou".contains(x)).collect::<Vec<_>>();
        let mut s = s.chars().collect::<Vec<_>>();
        val.sort();
        let mut j = 0;
        for i in 0..s.len()
        {
            if "AEIOUaeiou".contains(s[i])
            {
                s[i]=val[j];
                j+=1;
            }
        }
        s.into_iter().collect::<String>()
    }
}
