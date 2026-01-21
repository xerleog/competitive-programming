impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut ans = vec![0;58];
        word.as_bytes().into_iter().for_each(|x| ans[(x-65)as usize]+=1);
        (0..26).filter(|&x| ans[x as usize]>0 && ans[(x+32) as usize]>0).count() as i32   
    }
}
