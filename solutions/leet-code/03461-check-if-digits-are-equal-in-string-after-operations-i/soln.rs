impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut temp = s.as_bytes().windows(2).map(|w| (w[0]+w[1]-96)%10).collect::<Vec<_>>();
        while temp.len()>2
        {
            temp = temp.windows(2).map(|w| (w[0]+w[1])%10).collect::<Vec<_>>();
        }
        temp[0]==temp[1]
    }
}
