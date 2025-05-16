impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let mut ans = vec![0;26];
        s.as_bytes().into_iter().for_each(|x| ans[(x-97) as usize]+=1);
        let vow = "aeiou".as_bytes().into_iter().map(|x| ans[(x-97)as usize]).max().unwrap();
        "aeiou".as_bytes().into_iter().for_each(|x| ans[(x-97)as usize]=0);
        let cont = ans.into_iter().max().unwrap();
        vow+cont
    }
}
