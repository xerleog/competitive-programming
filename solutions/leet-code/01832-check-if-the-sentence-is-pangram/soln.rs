impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut ans = vec![0;26];
        sentence.as_bytes().into_iter().for_each(|x| ans[(x-97) as usize]+=1);
        ans.into_iter().all(|x| x>0)
    }
}
