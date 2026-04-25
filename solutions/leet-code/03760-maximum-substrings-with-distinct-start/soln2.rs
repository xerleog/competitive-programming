impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut ans = vec![0;26];
        s.as_bytes().into_iter().for_each(|x| ans[(x-97)as usize]+=1);
        ans.into_iter().filter(|&x| x>0).count() as i32
    }
}
